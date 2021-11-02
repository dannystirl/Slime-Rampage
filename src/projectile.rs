extern crate rogue_sdl;

use crate::Player;
use sdl2::rect::Rect;
use crate::gamedata::*;

pub enum ProjectileType{
	Bullet,
	Fireball,
}

pub struct Projectile{
	src: Rect,
	pos: Rect,
	pub facing_right: bool,
	is_active: bool,
	vector: Vec<f64>,
	pub p_type: ProjectileType,
	pub bounce_counter: i32,
}

 impl Projectile {
	pub fn new(pos: Rect, facing_right: bool, vector: Vec<f64>, p_type: ProjectileType) -> Projectile {
		let mut src = Rect::new(0 , 0 , TILE_SIZE, TILE_SIZE);
		let is_active = true;
		let bounce_counter = 0;
		Projectile {
			src,
			pos,
			facing_right,
			is_active,
			vector,
			p_type,
			bounce_counter,
		}
	}
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}

	pub fn y(&self) -> i32 {
		return self.pos.y;
	}

	pub fn set_xvel(&mut self, vel: f64) {
		self.vector[0] = vel;
	}

	pub fn set_yvel(&mut self, vel: f64) {
		self.vector[1] = vel;
	}

	pub fn xvel(&self) -> f64 {
		return self.vector[0];
	}

	pub fn yvel(&self) -> f64 {
		return self.vector[1];
	}
	
	 pub fn set_x(&mut self, x: i32){
		 self.pos.x = x;
	 }
	 pub fn set_y(&mut self, y: i32){
		 self.pos.y = y;
	 }
	pub fn is_active(&self) -> bool{
		return self.is_active;
	}
	// the frames aren't calculating right so the fireball image doesnt look right, but the logic is there.
	pub fn check_bounce(&mut self, xbounds:(i32,i32), ybounds: (i32,i32)){
		if self.get_bounce() >= 4 {
			self.die();
		}
		if self.x() <= xbounds.0 && self.is_active() {
			self.set_xvel( -self.xvel() );
			self.inc_bounce();
		}
		if self.x() >= xbounds.1 && self.is_active() {
			self.set_xvel( -self.xvel() );
			self.inc_bounce();
		}
		if self.y() <= ybounds.0 && self.is_active() {
			self.set_yvel( -self.yvel() );
			self.inc_bounce();
		}
		if self.y() >= ybounds.1 && self.is_active() {
			self.set_yvel( -self.yvel() );
			self.inc_bounce();
		}
	}
	pub fn update_pos(&mut self) {
		self.set_x(self.x() + self.vector[0] as i32);
		self.set_y(self.y() + self.vector[1] as i32);
		
	}
	pub fn set_pos(&mut self, p:Rect){
		self.pos = p;
	}
	pub fn src(&self) -> Rect{
		return self.src;
	}
	 pub fn die(&mut self){
		 // Set death animation when created
		 self.is_active = false;
	}
    pub fn pos(&self) -> Rect {
		self.pos
    }
	pub fn offset_pos(&self, player:&Player)-> Rect{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
		self.y() as i32 + (CENTER_H - player.y() as i32),
		TILE_SIZE, TILE_SIZE);
	}
	pub fn inc_bounce(&mut self) {
		self.bounce_counter += 1;
	}

	pub fn get_bounce(&mut self) -> i32 {
		return self.bounce_counter;
	}


}
