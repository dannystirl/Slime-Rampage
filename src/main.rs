extern crate rogue_sdl;

mod credits;

use std::time::Duration;
use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;

const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

const TILE_SIZE: u32 = 32;

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

pub struct SDL07 {
	core: SDLCore,
}

impl Game for SDL07 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL07{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();

		let w = 25;
		let mut x_pos = (CAM_W/2 - w/2) as i32;
		let mut y_pos = (CAM_H/2 - w/2) as i32;

		let mut x_vel = 0;
		let mut y_vel = 0;

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
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
			}

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);

			// Don't exceed speed limit
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			// Stay inside the viewing window
			x_pos = (x_pos + x_vel).clamp(0, (CAM_W - w) as i32);
			y_pos = (y_pos + y_vel).clamp(0, (CAM_H - w) as i32);

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

            let src = Rect::new(10, 10, TILE_SIZE * 2, TILE_SIZE * 2);
            let texture = texture_creator.load_texture("images/slime_r.png")?;
            let player_cam_pos = Rect::new(
				100 - 50,
				100 - 50,
				TILE_SIZE * 2,
				TILE_SIZE * 2,
			);
            self.core.wincan.copy(&texture, src, player_cam_pos)?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

struct Player<'a> {
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    fn new(texture: Texture<'a>) -> Player {
        Player {
            texture,
        }
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, SDL07::init);
    // credits::run_credits();
    Ok(())
}
