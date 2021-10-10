extern crate rogue_sdl;
mod enemy;
mod player;
mod RangedAttack;
mod credits;

use std::time::Duration;
use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
//use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 3;

pub struct SDL07 {
	core: SDLCore,
}

fn resist(vel: i32, deltav: i32) -> i32 {
	if deltav == 0 {
		if vel > 0 {
			-1
		}
		else if vel < 0 {
			1
		}
		else {
			deltav
		}
	}
	else {
		deltav
	}
}

impl Game for SDL07 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL07{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        // reset frame
        let texture_creator = self.core.wincan.texture_creator();
		let w = 25;
		
        // create sprites
		let mut p = player::Player::new(
			Rect::new(
				0, 0, TILE_SIZE, TILE_SIZE,
			),
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/player/blue_slime_l.png")?,
			texture_creator.load_texture("images/player/blue_slime_r.png")?,
		);
		p.set_x((CAM_W/2 - w/2) as i32);
		p.set_y((CAM_H/2 - w/2) as i32);
		p.set_x_vel(0);
		p.set_y_vel(0);
		
        let mut e = enemy::Enemy::new(
            Rect::new(
                (CAM_W/2 - TILE_SIZE/2) as i32,
                (CAM_H/2 - TILE_SIZE/2) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
            texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
        );
	
        let mut fireball = RangedAttack::RangedAttack::new(
			Rect::new(
				0, 0, TILE_SIZE, TILE_SIZE,
			),
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
            texture_creator.load_texture("images/fireball/fireball.png")?,
		);

		let mut ability1_frame = 0;
		let mut ability1_use = false;
		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			let mut x_deltav = 0;
			let mut y_deltav = 0;
            

            // move up
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
			}
            // move left
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
                p.facing_left = true;
			}
            // move down
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
            // move right
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
                p.facing_left = false;
			}
            // shoot fireball
            if keystate.contains(&Keycode::F) && ability1_frame==0{
				ability1_use=true; 
				fireball.start_pos(p.x(), p.y());
				println!("{}", fireball.x());
			}
			if ability1_use==true {
				ability1_frame=ability1_frame+1; 
				fireball.update_RangedAttack_pos(ability1_frame, (0, (CAM_W - TILE_SIZE) as i32));
				if ability1_frame==28 {
					ability1_use=false; 
					ability1_frame=0;
				}
			}

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(p.x_vel(), x_deltav);
			y_deltav = resist(p.y_vel(), y_deltav);

			// Don't exceed speed limit
			p.set_x_vel((p.x_vel() + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT));
			p.set_y_vel((p.y_vel() + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT));

			// Stay inside the viewing window
			p.set_x((p.x() + p.x_vel()).clamp(0, (CAM_W - w) as i32));
			p.set_y((p.y() + p.y_vel()).clamp(0, (CAM_H - w) as i32));

            p.update_pos((0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			//e.update_enemy_pos((x_vel, y_vel), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			
			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

            let background = texture_creator.load_texture("images/background/black_background.jpg")?;
            self.core.wincan.copy(&background, None, None)?;

            /* let cur_bg = Rect::new(
				((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (CAM_W * 2 - CAM_W) as i32),
				((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (CAM_H * 2 - CAM_H) as i32),
				CAM_W,
				CAM_H,
			); */

			self.core.wincan.copy(e.txtre(), e.src(), e.pos())?;

			if*(p.facing_left()) {
                self.core.wincan.copy(p.texture_l(), p.src(), p.pos())?;
                if ability1_use {self.core.wincan.copy(fireball.txtre(), fireball.src(ability1_frame, 4, 7), fireball.pos())?;}
            } else {
                self.core.wincan.copy(p.texture_r(), p.src(), p.pos())?;
                if ability1_use {self.core.wincan.copy(fireball.txtre(), fireball.src(ability1_frame, 4, 7), fireball.pos())?;}
            }
			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, SDL07::init);
    // credits::run_credits();
    Ok(())
}
