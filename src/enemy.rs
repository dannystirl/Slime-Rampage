extern crate rogue_sdl;

use std::time::Instant;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::f64;

const TILE_SIZE: u32 = 64;
#[allow(dead_code)]
const STUN_TIME: u32 = 2000;
const FIRE_COOLDOWN: u128 = 1500;

pub struct Enemy<'a> {
	vel: Rect, 
	pub pos: Rect,
	src: Rect,
	txtre: Texture<'a>,
	stun_timer: Instant,
	fire_timer: Instant,
	knockback_vel: f64,
	angle: f64,
	pub has_money: bool,
	pub x_flipped: bool,
	pub y_flipped: bool,
	pub facing_right: bool,
	pub is_stunned: bool,
	pub hp: i32,
	pub alive: bool,
	pub is_firing: bool,
	pub enemy_type: String,
	pub enemy_number: usize,
}

 impl<'a> Enemy<'a> {
	pub fn new(pos: Rect, txtre: Texture<'a>, enemy_type: String, num: usize) -> Enemy<'a> {
		let vel = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let stun_timer = Instant::now();
		let fire_timer = Instant::now();
		let knockback_vel = 0.0;
		let angle = 0.0;
		let x_flipped = false;
		let has_money = true;
		let y_flipped = false;
		let facing_right = false;
		let is_stunned = false;
		let is_firing =false;
		let hp = 10;
		let alive = true;
		let enemy_type = enemy_type;
		let enemy_number = num-1; 
		Enemy {
			vel, 
			pos,
			src,	
			txtre,
			stun_timer,
			fire_timer,
			knockback_vel,
			angle,
			has_money,
			x_flipped,
			y_flipped,
			facing_right,
			is_stunned,
			hp,
			alive,
			is_firing,
			enemy_type, 
			enemy_number, 
		}
	}

	// x values
	pub fn set_x(&mut self, x:f64){
		self.pos.x = x as i32;
	}
	pub fn x(&self) -> f64 {
		return self.pos.x.into();
	}
	pub fn set_x_vel(&mut self, x:f64){
		self.vel.x = x as i32;
	}
	pub fn x_vel(&self) -> f64 {
		return self.vel.x.into();
	}
	pub fn width(&self) -> u32 {
		self.pos.width()
	}

	// y values
	pub fn set_y(&mut self, y:f64){
		self.pos.y = y as i32;
	}
	pub fn y(&self) -> f64 {
		return self.pos.y.into();
	}
	pub fn set_y_vel(&mut self, y:f64){
		self.vel.y = y as i32;
	}
	pub fn y_vel(&self) -> f64 {
		return self.vel.y.into();
	}
	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	// movement stuff
	pub fn update_pos(&mut self, roll:i32, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		if self.is_stunned {
			return;
		}
		if roll == 1 {
			self.pos.set_x((self.x() as i32 + (self.x_vel() as i32) + 1).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() as i32 + self.y_vel() as i32).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 2 {
			self.pos.set_x((self.x() as i32 + self.x_vel() as i32).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() as i32 + (self.y_vel() as i32) + 1).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 3 {
			self.pos.set_x((self.x() as i32 + self.x_vel() as i32).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() as i32 + (self.y_vel() as i32) - 1).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 4 {
			self.pos.set_x((self.x() as i32 + (self.x_vel() as i32) - 1).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() as i32 + self.y_vel() as i32).clamp(y_bounds.0, y_bounds.1));
		}
	}

	pub fn aggro(&mut self, player_pos_x: f64, player_pos_y: f64, x_bounds: (i32, i32), y_bounds: (i32, i32), speed_limit_adj: f64) {
		if self.is_stunned {
			return;
		}
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let mut x = speed_limit_adj * angle.sin();
		if vec[0] < 0.0 {
			x *= -1.0;
		}
		let mut y = speed_limit_adj * angle.cos();
		if vec[1] < 0.0  {
			y *= -1.0;
		}
		self.pos.set_x(((self.x() + x) as i32).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y(((self.y() + y) as i32).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn flee(&mut self, player_pos_x: f64, player_pos_y: f64, x_bounds: (i32, i32), y_bounds: (i32, i32), speed_limit_adj: f64) {
		if self.is_stunned {
			return;
		}
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let mut x = speed_limit_adj / 1.5 as f64 * angle.sin();
		if vec[0] >= 0.0 {
			x *= -1.0;
		}
		let mut y = speed_limit_adj / 1.5 as f64 * angle.cos();
		if vec[1] >= 0.0  {
			y *= -1.0;
		}
		self.pos.set_x(((self.x() + x) as i32).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y(((self.y() + y) as i32).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn knockback(&mut self, player_pos_x: f64, player_pos_y: f64, x_bounds: (i32, i32), y_bounds: (i32, i32))
	{
		self.x_flipped = false;
		self.y_flipped = false;
		self.is_stunned = true;
		self.knockback_vel = 4.0;
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		self.angle = angle;
		let mut x = -5.0 * angle.sin();
		if vec[0] < 0.0 {
			x *= -1.0;
			self.x_flipped = true;
		}
		let mut y = -5.0 * angle.cos();
		if vec[1] < 0.0 {
			y *= -1.0;
			self.y_flipped = false;
		}
		self.pos.set_x(((self.x() + x) as i32).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y(((self.y() + y) as i32).clamp(y_bounds.0, y_bounds.1));
		self.stun_timer = Instant::now();
	}

	pub fn src(&self) -> Rect {
		self.src
	}

    pub fn txtre(&self) -> &Texture {
        &self.txtre
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn get_vel(&self) -> f64 {
		self.knockback_vel
	}

	pub fn slow_vel(&mut self, decel: f64) {
		self.knockback_vel -= decel;
	}

	pub fn angle(&self) -> f64 {
		self.angle
	}

	// attacking 
	pub fn get_fire_timer(&self) -> u128 {
		self.fire_timer.elapsed().as_millis()
	}

	pub fn fire(&mut self){
		if self.get_fire_timer() < FIRE_COOLDOWN {
		 return;
		}
		self.is_firing = true;
		self.fire_timer = Instant::now();
		
	}	

	pub fn get_fire_cooldown(&self)-> u128{
		FIRE_COOLDOWN
	}
	pub fn set_fire_cooldown(&mut self){
		self.is_firing =false;
	}

	// health
	pub fn get_stun_timer(&self) -> u128 {
		self.stun_timer.elapsed().as_millis()
	}

	pub fn set_stunned(&mut self, stunned: bool) {
		self.is_stunned = stunned;
	}

	pub fn minus_hp(&mut self, dmg: i32) {
		self.hp -= dmg;

		if self.hp <= 0 {
			self.die();
		}
	}

	pub fn die(&mut self){
		// Set death animation when created
		self.alive = false;
	}

	pub fn is_alive(&mut self) -> bool{
		return self.alive;
	}

	pub fn has_gold(&mut self) -> bool{
		return self.has_money;
	}

	pub fn set_no_gold(&mut self) {
		self.has_money = false;
	}
}
