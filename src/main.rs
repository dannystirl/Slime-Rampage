extern crate rogue_sdl;
mod enemy;
mod player;
mod ranged_attack;
mod credits;

use std::collections::HashSet;
use rand::Rng;
use crate::enemy::*;
use crate::ranged_attack::*;
use crate::player::*;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
//use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;

// window globals
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;

// game globals
const SPEED_LIMIT: i32 = 3;
const ACCEL_RATE: i32 = 3;


pub struct ROGUELIKE {
	core: SDLCore,
}

// calculate velocity resistance
fn resist(vel: i32, delta: i32) -> i32 {
	if delta == 0 {
		if vel > 0 {-1}
		else if vel < 0 {1}
		else {delta}
	} else {delta}
}

// check collision
fn check_collision(a: &Rect, b: &Rect) -> bool {
	if a.bottom() < b.top()
		|| a.top() > b.bottom()
		|| a.right() < b.left()
		|| a.left() > b.right()
	{
		false
	}
	else {
		true
	}
}

// CREATE GAME
impl Game for ROGUELIKE {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(ROGUELIKE{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        // reset frame
        let texture_creator = self.core.wincan.texture_creator();
		let screen_width = 25;
		
		let mut rng = rand::thread_rng();
		let mut count = 0;
		let f_display = 15;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		let mut player = player::Player::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/player/blue_slime_l.png")?,
			texture_creator.load_texture("images/player/blue_slime_r.png")?,
			texture_creator.load_texture("images/player/slime_animation_l.png")?,
			texture_creator.load_texture("images/player/slime_animation_r.png")?,
		);

		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		let mut enemies: Vec<Enemy> = Vec::with_capacity(2);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement
		let mut i=1; 
		for _ in 0..enemies.capacity(){
			let e = enemy::Enemy::new(
				Rect::new(
					(CAM_W/2 - TILE_SIZE/2 + 100) as i32,
					(CAM_H/2 - TILE_SIZE/2 + 100) as i32,
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
			);
			enemies.push(e);
			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}
		
		// CREATE FIREBALL (SHOULD BE MOVED TO fireball.rs WHEN CREATED)
        let mut fireball = ranged_attack::RangedAttack::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			false,
			false,
			0,
            texture_creator.load_texture("images/fireball/fireball.png")?,
		);

		// MAIN GAME LOOP
		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			player.set_x_delta(0);
			player.set_y_delta(0);

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

				// FOR TESTING ONLY: USE TO FOR PRINT VALUES
				if keystate.contains(&Keycode::P) {
					println!("x:{} y:{} ", enemies[0].x(), enemies[0].y());
				}

			ROGUELIKE::check_inputs(&mut fireball, keystate, &mut player);
			ROGUELIKE::update_player(&screen_width, &mut player);
			ROGUELIKE::check_collisions(&mut player, &mut enemies);
			if player.is_dead(){
				break 'gameloop;
			}

			// SET BACKGROUND
            let background = texture_creator.load_texture("images/background/bb.png")?;
            self.core.wincan.copy(&background, None, None)?;

			// UPDATE ENEMIES
			rngt = ROGUELIKE::update_enemies(self, rngt, &mut enemies);

			// Should be switched to take in array of active fireballs, bullets, etc.
			self.update_projectiles(&mut fireball);
			self.draw_player(&count, &f_display, &mut player);
			count = count + 1;
			if count > f_display * 5 {
				count = 0;
			}

			// UPDATE FRAME
			self.core.wincan.present();

		}
		// Out of game loop, return Ok
		Ok(())
	}
}


pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
	//credits::run_credits()
	Ok(())
}

// update enemies
impl ROGUELIKE {
	pub fn update_enemies(&mut self, mut rngt: Vec<i32>, enemies: &mut Vec<Enemy>) -> Vec<i32>{
		let mut i=1;
		let mut rng = rand::thread_rng();
		for enemy in enemies {
			if rngt[0] > 47 || ROGUELIKE::check_edge(&enemy){
				rngt[i] = rng.gen_range(1..5);
				rngt[0] = 0;
			}
			enemy.update_pos(rngt[i], (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
			self.core.wincan.copy(enemy.txtre(), enemy.src(), enemy.pos()).unwrap();
			i+=1;
		}
		rngt[0]+=1;
		return rngt;
	}
}

// check input values
impl ROGUELIKE {
	pub fn check_inputs(fireball: &mut RangedAttack, keystate: HashSet<Keycode>, mut player: &mut Player) {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - ACCEL_RATE);
			player.is_still = false;
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - ACCEL_RATE);
			player.facing_left = true;
			player.facing_right = false;
			player.is_still = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + ACCEL_RATE);
			player.is_still = false;
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + ACCEL_RATE);
			player.facing_left = false;
			player.facing_right = true;
			player.is_still = false;
		}

		// shoot fireball
		if keystate.contains(&Keycode::F) && fireball.frame() == 0 {
			fireball.set_use(true);
			fireball.start_pos(player.x(), player.y(), player.facing_left);
		}
	}
}

// update projectiles
impl ROGUELIKE {
	pub fn update_projectiles(&mut self, fireball: &mut RangedAttack) {
		if fireball.in_use() {
			fireball.set_frame(fireball.frame() + 1);
			fireball.update_pos((0, (CAM_W - TILE_SIZE) as i32));
			if fireball.frame() == 28 {
				fireball.set_use(false);
				fireball.set_frame(0);
			}
			// this needs to be mirrored
			self.core.wincan.copy(fireball.texture(), fireball.src(4, 7), fireball.pos()).unwrap();
		}
	}
}

// check collisions
impl ROGUELIKE {
	fn check_collisions(player: &mut Player, enemies: &mut Vec<Enemy>) {
		for enemy in enemies {
			if check_collision(&player.pos(), &enemy.pos())
				|| player.pos().left() < 0
				|| player.pos().right() > CAM_W as i32
			{
				player.set_x(player.x() - player.x_vel());
				player.minus_hp(0.2);
				println!("hp = {}", player.get_hp());
			}

			if check_collision(&player.pos(), &enemy.pos())
				|| player.pos().top() < 0
				|| player.pos().bottom() > CAM_H as i32
			{
				player.set_y(player.y() - player.y_vel());
				player.minus_hp(0.2);
				println!("hp = {}", player.get_hp());
			}
		}
	}
}

// update player
impl ROGUELIKE {
	fn update_player(w: &u32, mut player: &mut Player) {
		// Slow down to 0 vel if no input and non-zero velocity
		player.set_x_delta(resist(player.x_vel(), player.x_delta()));
		player.set_y_delta(resist(player.y_vel(), player.y_delta()));

		//when player is not moving
		if player.x_vel() == 0 && player.y_vel() == 0 { player.is_still = true; }//?

		// Don't exceed speed limit
		player.set_x_vel((player.x_vel() + player.x_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));
		player.set_y_vel((player.y_vel() + player.y_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));

		// Stay inside the viewing window
		player.set_x((player.x() + player.x_vel()).clamp(0, (CAM_W - w) as i32));
		player.set_y((player.y() + player.y_vel()).clamp(0, (CAM_H - w) as i32));

		player.update_pos((0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));
	}
}

// draw player
impl ROGUELIKE {
	pub fn draw_player(&mut self, count: &i32, f_display: &i32, player: &mut Player) {
		if *(player.is_still()) {
			if *(player.facing_right()) {
				self.core.wincan.copy(player.texture_a_r(), player.src(), player.pos()).unwrap();
			} else {
				self.core.wincan.copy(player.texture_a_l(), player.src(), player.pos()).unwrap();
			}

			//display animation when not moving
			match count {
				count if count < f_display => { player.set_src(0 as i32, 0 as i32); }
				count if count < &(f_display * 2) => { player.set_src(TILE_SIZE as i32, 0 as i32); }
				count if count < &(f_display * 3) => { player.set_src(0 as i32, TILE_SIZE as i32); }
				count if count < &(f_display * 4) => { player.set_src(TILE_SIZE as i32, TILE_SIZE as i32); }
				_ => { player.set_src(0, 0); }
			}
		} else {
			player.set_src(0, 0);
			if *(player.facing_right()) {
				self.core.wincan.copy(player.texture_r(), player.src(), player.pos()).unwrap();
			} else {
				self.core.wincan.copy(player.texture_l(), player.src(), player.pos()).unwrap();
			}
		}
	}
}

// force enemy movement
impl ROGUELIKE {
	pub fn check_edge(enemy: &enemy::Enemy) -> bool{
		if  enemy.x() == 0 || 
		enemy.x() + (enemy.width() as i32) ==  ((CAM_W - TILE_SIZE) as i32) ||
		enemy.y() - (enemy.height() as i32) == 0 || 
		enemy.y() == ((CAM_H - TILE_SIZE) as i32)
		{return true;}
		else {return false;}
	}
}