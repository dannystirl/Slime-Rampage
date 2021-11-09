extern crate rogue_sdl;

use crate::Player;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use crate::gamedata::*;
use crate::projectile::Direction::{Down, Up, Left, Right};
use crate::player::*;

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
		let src = Rect::new(0 , 0 , TILE_SIZE, TILE_SIZE);
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

	pub fn set_x_vel(&mut self, vel: f64) {
		self.vector[0] = vel;
	}

	pub fn set_y_vel(&mut self, vel: f64) {
		self.vector[1] = vel;
	}

	pub fn x_vel(&self) -> f64 {
		return self.vector[0];
	}

	pub fn y_vel(&self) -> f64 {
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
	pub fn check_bounce(&mut self, xbounds:(i32,i32), ybounds: (i32,i32), map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]){
			match self.p_type{
				ProjectileType::Fireball=>{
					if self.get_bounce() >= 1 {
						self.die();
					}
				}
				_ =>{
					if self.get_bounce() >= 4 {
						self.die();
					}
				}
			}	
		if !DEVELOP {
			if self.x() <= xbounds.0 && self.is_active() {
				self.set_x_vel( -self.x_vel() );
				self.inc_bounce();
			}
			if self.x() >= xbounds.1 && self.is_active() {
				self.set_x_vel( -self.x_vel() );
				self.inc_bounce();
			}
			if self.y() <= ybounds.0 && self.is_active() {
				self.set_y_vel( -self.y_vel() );
				self.inc_bounce();
			}
			if self.y() >= ybounds.1 && self.is_active() {
				self.set_y_vel( -self.y_vel() );
				self.inc_bounce();
			}

		} else {
			let h_bounds_offset = (self.y() / TILE_SIZE as i32) as i32;
			let w_bounds_offset = (self.x() / TILE_SIZE as i32) as i32;
			let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);
	
			for h in 0..(CAM_H / TILE_SIZE) + 1 {
				for w in 0..(CAM_W / TILE_SIZE) + 1 {
	
				let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as i32) as i32 - (CENTER_W - self.x() as i32),
				(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as i32) as i32 - (CENTER_H - self.y() as i32),
				TILE_SIZE, TILE_SIZE);
		
				if h as i32 + h_bounds_offset < 0 ||
				   w as i32 + w_bounds_offset < 0 ||
				   h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				   w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				   map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
				continue;
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 {
					let p_pos = self.pos();
	
					if GameData::check_collision(&p_pos, &w_pos) {
							//println!("c");
							collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
						}
					}
				}
			}
			self.resolve_col(&collisions);

		}
		
	}

	pub fn collect_col(&mut self, p_pos: Rect, p_center: Point, other_pos :Rect) -> CollisionDecider {
		let distance = ((p_center.x() as f64 - other_pos.center().x() as f64).powf(2.0) + (p_center.y() as f64 - other_pos.center().y() as f64).powf(2.0)).sqrt();

		// player above other
		if p_pos.bottom() >= other_pos.top() && p_center.y() < other_pos.top(){
			let resolution = CollisionDecider::new(Down, distance as i32);
			return resolution;
		}
		// player left of other
		if p_pos.right() >= other_pos.left() && p_center.x() < other_pos.left() {
			let resolution = CollisionDecider::new(Right, distance as i32);
			return resolution;
		}
		// player below other
		if p_pos.top() <= other_pos.bottom() && p_center.y() > other_pos.bottom(){
			let resolution = CollisionDecider::new(Up, distance as i32);
			return resolution;
		}
		// player right of other
		 else {
			 let resolution = CollisionDecider::new(Left, distance as i32);
			 return resolution;
		}
	}

	pub fn resolve_col(&mut self, collisions : &Vec<CollisionDecider>){
		// Sort vect of collisions by distance
		let mut sorted_collisions: Vec<CollisionDecider> = Vec::new();
		for c in collisions{
			let new_dir = &c.dir;
			sorted_collisions.push(CollisionDecider::new(*new_dir,c.dist) );
		}
		sorted_collisions.sort_by_key(|x| x.dist);

		// Handle collisions based on distance
		if sorted_collisions.len() > 0 {
			match sorted_collisions[0].dir{
				Direction::Up=>{
					self.set_y_vel(-self.y_vel());
					self.inc_bounce();
				}
				Direction::Down=>{
					self.set_y_vel(-self.y_vel());
					self.inc_bounce();
					
				}
				Direction::Right=>{
					self.set_x_vel(-self.x_vel());
					self.inc_bounce();
				
				}
				Direction::Left=>{
					self.set_x_vel(-self.x_vel());
					self.inc_bounce();
					
				}
				Direction::None=>{
					println!("I have no clue how this happened");
				}
			}
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
		return Rect::new(self.x() as i32, //screen coordinates
		self.y() as i32,
		TILE_SIZE / 2, TILE_SIZE / 2);
    }
	pub fn offset_pos(&self, player:&Player)-> Rect{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
		self.y() as i32 + (CENTER_H - player.y() as i32),
		TILE_SIZE / 2, TILE_SIZE / 2);
	}
	pub fn inc_bounce(&mut self) {
		self.bounce_counter += 1;
	}

	pub fn get_bounce(&mut self) -> i32 {
		return self.bounce_counter;
	}
}
