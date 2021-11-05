extern crate rogue_sdl;

use std::time::Instant;
use sdl2::rect::{Rect, Point};
use sdl2::render::{Texture};
use sdl2::image::LoadTexture;
use crate::projectile;
use crate::projectile::*;
use crate::gamedata::GameData;
use crate::gamedata::*;
use crate::SDLCore;
pub enum Direction{
	Up,
	Down,
	Left,
	Right,
	None,
}
pub enum Ability{
	Bullet,
}

pub enum Weapon{
	Sword,
}

pub struct Player<'a> {
	pos: (f64, f64),
	cam_pos: Rect,
	mass: f64,
	vel: (i32, i32),
	delta: (i32, i32),
	height: u32,
	width: u32,
	src: Rect,
	attack_box: Rect,
	attack_timer: Instant,
	fire_timer: Instant,
	damage_timer: Instant,
	mana_timer: Instant,
	texture_all: Texture<'a>,
	invincible: bool,
	pub facing_right: bool,
	pub hp: u32,
	pub mana: i32,
	pub max_mana: i32,
	pub is_attacking: bool,
	pub weapon_frame: i32,	
	pub is_firing: bool,
	pub coins: u32,
	pub weapon: Weapon,
	pub ability: Ability,
}

impl<'a> Player<'a> {
	pub fn new(pos: (f64, f64), texture_all: Texture<'a>) -> Player<'a> {
		let cam_pos = Rect::new(
			0,
			0,
			TILE_SIZE,
			TILE_SIZE,
		);
		let mass = 1.5;
		let vel = (0, 0);
		let delta = (0, 0);
		let height = TILE_SIZE; // 32;
		let width = TILE_SIZE; // 32;
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let hp = 30;
		let mana = 4;
		let max_mana = 4;
		let facing_right = false;
		let is_attacking = false;
		let is_firing =false;
		let attack_box = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let attack_timer = Instant::now();
		let fire_timer = Instant::now();
		let damage_timer = Instant::now();
		let mana_timer = Instant::now();
		let invincible = true;
		let weapon_frame=0; 
		let coins = 0;
		let weapon = Weapon::Sword;
		let ability = Ability::Bullet;
		Player {
			pos,
			cam_pos,
			mass,
			vel,
			delta,
			height,
			width,
			src,
			attack_box,
			attack_timer,
			fire_timer,
			damage_timer,
			mana_timer,
			invincible,
			texture_all,
			facing_right,
			hp,
			mana,
			max_mana,
			is_attacking,
			weapon_frame,
			is_firing,
			coins,
			weapon,
			ability,
		}
	}

	// update player
	pub fn update_player(&mut self, game_data: &GameData, mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], core: &mut SDLCore) -> Result<(), String>  {
		let tc = core.wincan.texture_creator();
		let hitbox =tc.load_texture("images/objects/crate.png")?;
		let xwalls = game_data.rooms[0].xwalls;
		let ywalls = game_data.rooms[0].ywalls;
		let speed_limit_adj = game_data.get_speed_limit();
		
		// Slow down to 0 vel if no input and non-zero velocity
		self.set_x_delta(resist(self.x_vel() as i32, self.x_delta() as i32));
		self.set_y_delta(resist(self.y_vel() as i32, self.y_delta() as i32));

		// Don't exceed speed limit
		self.set_x_vel((self.x_vel() + self.x_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
		self.set_y_vel((self.y_vel() + self.y_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
		
		// Stay inside the viewing window
		//self.set_x((self.x() + self.x_vel() as f64));//.clamp(0.0, (xwalls.1 * TILE_SIZE as i32) as f64) as f64);
		//self.set_y((self.y() + self.y_vel() as f64));//.clamp(0.0, (ywalls.1 * TILE_SIZE as i32) as f64) as f64);
		let src = Rect::new(0, 0, TILE_SIZE/4, TILE_SIZE/4);

		let h_bounds_offset = (self.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as f64) as i32;
		let mut collisions: Vec<Direction> = Vec::with_capacity(5);

		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {
				
					
					let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32 - (CENTER_W - self.x() as i32),
					(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32 - (CENTER_H - self.y() as i32),
					TILE_SIZE, TILE_SIZE);
	
					let debug_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32,// - (CENTER_W - self.x() as i32),
					(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32,// - (CENTER_H - self.y() as i32),
					TILE_SIZE, TILE_SIZE);
					if h as i32 + h_bounds_offset < 0 ||
				   w as i32 + w_bounds_offset < 0 ||
				   h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				   w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				   map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
					} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 {
						let p_pos = self.pos();

						core.wincan.copy(&hitbox, src, w_pos);
						if GameData::check_collision(&p_pos, &w_pos) {//I hate collisions
							core.wincan.copy(&hitbox, src, self.cam_pos);

							core.wincan.copy(&hitbox, src, debug_pos);
							collisions.push(self.resolve_col(p_pos, self.pos().center(), w_pos));
							
							//println!("welcome to hell");
							// NW
						}

					}
			}
		}

		self.resolve_col_further(collisions);

		for c in &game_data.crates{
			let crate_pos = c.pos();
			let p_pos =self.pos();
		
			if GameData::check_collision(&self.pos(), &c.pos()) {//I hate collisions
				//println!("welcome to hell");
				self.resolve_col(self.pos(), self.pos().center(), c.pos());
			}
		}
		self.update_pos((-100 * TILE_SIZE as i32, 100 * TILE_SIZE as i32), (-100 * TILE_SIZE as i32, 100 * TILE_SIZE as i32));/* game_data.rooms[0].xbounds, game_data.rooms[0].ybounds */
		// is the player currently attacking?
		if self.is_attacking { self.set_attack_box(self.x() as i32, self.y() as i32); }
		if self.get_attack_timer() > ATTK_COOLDOWN {
			self.is_attacking = false;
			// clear attack box
			self.attack_box = Rect::new(self.x() as i32, self.y() as i32, 0, 0);
		}
		// is the player currently firing?
		if self.fire_timer.elapsed().as_millis() > FIRE_COOLDOWN_P {
			self.is_firing =false;
		}

		self.restore_mana();
		Ok(())
	}

	// player x values
	pub fn set_x(&mut self, x: f64){
		self.pos.0 = x;
	}
	pub fn x(&self) -> f64 {
		return self.pos.0;
	}
	pub fn set_x_vel(&mut self, x: i32){
		self.vel.0 = x;
	}
	pub fn x_vel(&self) -> i32 {
		return self.vel.0;
	}
	pub fn set_x_delta(&mut self, x: i32){
		self.delta.0 = x;
	}
	pub fn x_delta(&self) -> i32 {
		return self.delta.0;
	}
	pub fn width(&self) -> u32 {
		self.width
	}

	// player y values
	pub fn set_y(&mut self, y: f64){
		self.pos.1 = y;
	}
	pub fn y(&self) -> f64 {
		return self.pos.1;
	}
	pub fn set_y_vel(&mut self, y: i32){
		self.vel.1 = y;
	}
	pub fn y_vel(&self) -> i32 {
		return self.vel.1;
	}
	pub fn set_y_delta(&mut self, y: i32){
		self.delta.1 = y;
	}
	pub fn y_delta(&self) -> i32 {
		return self.delta.1;
	}
	pub fn height(&self) -> u32 {
		self.height
	}

	// update position
	pub fn update_pos(&mut self, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.0 = (self.x() + self.x_vel() as f64 * 2.0 )/* .clamp(x_bounds.0 as f64, x_bounds.1 as f64) */;
		self.pos.1 = (self.y() + self.y_vel() as f64 * 2.0)/* .clamp(y_bounds.0 as f64, y_bounds.1 as f64) */;
	}

	pub fn set_src(&mut self, x: i32, y: i32) {
		self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn pos(&self) -> Rect {
        return Rect::new(
			self.x() as i32,
			self.y() as i32,
			TILE_SIZE/2,
			TILE_SIZE/2,
		)
    }

	pub fn set_cam_pos(&mut self, x: i32, y: i32) {
		self.cam_pos = Rect::new(
			self.x() as i32 - x,
			self.y() as i32 - y,
			TILE_SIZE/2,
			TILE_SIZE/2,
		);
	}

	pub fn get_cam_pos(&self) -> Rect {
        self.cam_pos
    }

	pub fn get_mass(&self) -> f64 { self.mass }

	pub fn texture_all(&self) -> &Texture {
        &self.texture_all
    }

	pub fn get_frame_display(&mut self, count: &i32, f_display: &i32) {
		if count < &f_display { self.set_src(0 as i32, 0 as i32); }
		else if count < &(f_display * 2) { self.set_src(64 as i32, 0 as i32); }
		else if count < &(f_display * 3) { self.set_src(128 as i32, 0 as i32); }
		else if count < &(f_display * 4) { self.set_src(0 as i32, 64 as i32); }
		else if count < &(f_display * 5) { self.set_src(64 as i32, 64 as i32); }
		else if count < &(f_display * 6) { self.set_src(128 as i32, 64 as i32); }
		else if count < &(f_display * 7) { self.set_src(0 as i32, 128 as i32); }
		else if count < &(f_display * 8) { self.set_src(64 as i32, 128 as i32); }
		else if count < &(f_display * 9) { self.set_src(128 as i32, 128 as i32); }
		else if count < &(f_display * 10) { self.set_src(0 as i32, 192 as i32); }
		else if count < &(f_display * 11) { self.set_src(64 as i32, 192 as i32); }
		else if count < &(f_display * 12) { self.set_src(128 as i32, 192 as i32); }
		else { self.set_src(0, 0); }
	}

	// attacking values
	pub fn get_attack_timer(&self) -> u128 {
		self.attack_timer.elapsed().as_millis()
	}

	pub fn get_attack_box(&self) -> Rect {
		self.attack_box
	}

	pub fn set_attack_box(&mut self, x: i32, y: i32) {
		if self.facing_right{
			self.attack_box = Rect::new(x + TILE_SIZE as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		} else {
			self.attack_box = Rect::new(x - ATTACK_LENGTH as i32, y as i32, ATTACK_LENGTH, TILE_SIZE);
		}
	}

	pub fn attack(&mut self) {
		if self.get_attack_timer() < ATTK_COOLDOWN {
			return;
		}
		self.is_attacking = true;
		self.set_attack_box(self.x() as i32, self.y() as i32);
		self.attack_timer = Instant::now();
	}

	pub fn fire(&mut self, mouse_x: i32, mouse_y: i32, speed_limit: f64, p_type: ProjectileType) -> Projectile {
			self.is_firing = true;
			self.use_mana();
			self.fire_timer = Instant::now();

			let vec = vec![mouse_x as f64 - CENTER_W as f64 - (TILE_SIZE / 2) as f64, mouse_y as f64 - CENTER_H as f64 - (TILE_SIZE / 2) as f64];
			let angle = ((vec[0] / vec[1]).abs()).atan();
			let speed: f64 = 3.0 * speed_limit;
			let mut x = &speed * angle.sin();
			let mut y = &speed * angle.cos();
			if vec[0] < 0.0 {
				x *= -1.0;
			}
			if vec[1] < 0.0 {
				y *= -1.0;
			}

			let p_type = p_type;
			let bullet = projectile::Projectile::new(
				Rect::new(
					self.x() as i32,
					self.y() as i32,
					TILE_SIZE / 2,
					TILE_SIZE / 2,
				),
				false,
				vec![x, y],
				p_type,
			);

			return bullet;
	}

	//mana values
	pub fn get_mana(&self) -> i32 {
		return self.mana
	}

	pub fn get_max_mana(&self) -> i32 {
		self.max_mana
	}

	pub fn get_mana_timer(&self) -> u128 {
		self.mana_timer.elapsed().as_millis()
	}

	pub fn use_mana(&mut self) {
		self.mana -= 1;
	}

	pub fn restore_mana(&mut self) {
		if self.get_mana_timer() < MANA_RESTORE_RATE || self.get_mana() >= self.max_mana {
			return;
		}

		self.mana += 1;
		self.mana_timer = Instant::now();
	}



	// heatlh values
	pub fn get_hp(&self) -> u32 {
		return self.hp
	}

	pub fn is_dead(&self) -> bool {
		return self.hp <= 0;
	}

	pub fn minus_hp(&mut self, dmg: u32) {
		if self.invincible {
			return;
		}
		self.hp -= dmg;
		self.damage_timer = Instant::now();
	}

	pub fn set_invincible(&mut self){
		if self.damage_timer.elapsed().as_millis() < DMG_COOLDOWN {
			 self.invincible = true;
		} else {
			self.invincible = false;
		}
	}

	//coin values
	pub fn get_coins(&self) -> u32 {
		return self.coins
	}

	pub fn add_coins(&mut self, coins_to_add: u32)  {
		self.coins += coins_to_add;
	}

	pub fn sub_coins(&mut self, coins_to_add: u32)  {
		self.coins -= coins_to_add;
	}

	pub fn resolve_col(&mut self, p_pos: Rect, p_center: Point, other_pos :Rect) -> Direction {
		// player above other

		if p_pos.bottom() >= other_pos.top() && p_center.y() < other_pos.top(){
			println!("bottom of player");
			return Direction::Down
		}	
		if p_pos.right() >= other_pos.left() && p_center.x() < other_pos.left(){
			println!("right of player");
			return Direction::Right
	   }
		if p_pos.top() <= other_pos.bottom() && p_center.y() > other_pos.bottom(){
			println!("top of player");
			return Direction::Up
		}
		// player right of other
		 if p_pos.left() <= other_pos.right() && p_center.x() > other_pos.right(){
			println!("left of player");
			return  Direction::Left
		}
		// player below object
		// player left of other
	 	return Direction::None;
	}
	pub fn resolve_col_further(&mut self, collisions : Vec<Direction>){
		if(collisions.len() > 0){
			match collisions[0]{
				Direction::Up=>{
					self.set_y_vel(self.y_vel().clamp(0,100));
					if collisions.len() > 2 {
						match collisions[2]{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
			
							}
							Direction::Down=>{
	
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Down=>{
					self.set_y_vel(self.y_vel().clamp(-100,0));
				
					if collisions.len() > 2 {
						match collisions[2]{
							Direction::Up=>{
	
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
	
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Right=>{
					self.set_x_vel(self.x_vel().clamp(-100,0));
	
					if collisions.len() > 2 {
						match collisions[2]{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
	
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
	
							}
							Direction::Left=>{
								//self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100,0));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Left=>{
					self.set_x_vel(self.x_vel().clamp(0,100));
					if collisions.len() > 2 {
						match collisions[1]{
							Direction::Up=>{
	
							}
							Direction::Down=>{
								
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::Right=>{
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
						match collisions[2]{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0,100));
	
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100,0));
	
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(0,100));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				
				Direction::None=>{
					println!("I have no clue how this happened");
				}
			}
		}
	}
}

// calculate velocity resistance
pub(crate) fn resist(vel: i32, delta: i32) -> i32 {
	if delta == 0 {
		if vel > 0 {-1}
		else if vel < 0 {1}
		else {delta}
	} else {delta}
}



