extern crate rogue_sdl;

use crate::Player;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use crate::gamedata::*;
use crate::projectile::Direction::{Down, Up, Left, Right};
use crate::player::*;
use crate::power::*;
use crate::crateobj::*;
use crate::rigidbody::Rigidbody;

pub struct Projectile{
	src: Rect,
	pub facing_right: bool,
	is_active: bool,
	pub power: Power,
	pub bounce_counter: i32,
	pub elapsed: u128,
	pub rb: Rigidbody,
	pub angle: f64,
	pub facing_up: bool,
}

impl Projectile {
	pub fn new(pos: Rect, facing_right: bool, velocity: Vec<f64>, power_type: PowerType, elapsed: u128, angle: f64) -> Projectile {
		let src = Rect::new(0 , 0 , TILE_SIZE, TILE_SIZE);
		let is_active = true;
		let bounce_counter = 0;
		let rb = Rigidbody::new(pos, velocity[0], velocity[1], 4.0, 0.0);
		let facing_up = false;
		let power = Power::new(pos, power_type); 
		Projectile {
			src,
			facing_right,
			is_active,
			bounce_counter,
			elapsed,
			rb,
			angle,
			facing_up,
			power, 
		}
	}
	pub fn x(&self) -> i32 {
		return self.rb.hitbox.x as i32;
	}

	pub fn y(&self) -> i32 {
		return self.rb.hitbox.y as i32;
	}

	pub fn set_x_vel(&mut self, vel: f64) {
		self.rb.vel.x = vel;
	}

	pub fn set_y_vel(&mut self, vel: f64) {
		self.rb.vel.y = vel;
	}

	pub fn x_vel(&self) -> f64 {
		return self.rb.vel.x;
	}

	pub fn y_vel(&self) -> f64 {
		return self.rb.vel.y;
	}

	 pub fn set_x(&mut self, x: i32){
		 self.rb.hitbox.x = x as f64;
	 }
	 pub fn set_y(&mut self, y: i32){
		 self.rb.hitbox.y = y as f64;
	 }
	pub fn is_active(&self) -> bool{
		return self.is_active;
	}
	
	// check object bouncing 
	pub fn check_bounce(&mut self, _crates: &mut Vec<Crate>, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]){
		match self.power.power_type {
			PowerType::Fireball => {
				if self.get_bounce() >= 1 {
					self.die();
				}
			}
			PowerType::Rock => {
				if self.get_bounce() >= 1 {
					self.die();
				}
			}
			PowerType::Shrapnel => {
				if self.get_bounce() >= 2 {
					self.die();
				}
			}
			_ => {
				if self.get_bounce() >= 4 {
					self.die();
				}
			}
		}

		// WALL COLLISIONS :D
		let mut wall_collisions: Vec<CollisionDecider> = Vec::with_capacity(69);


		let h_bounds_offset = (self.y() / TILE_SIZE as i32) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as i32) as i32;

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
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 ||
					map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 5 {
					if GameData::check_collision(&self.rb.pos(), &w_pos) {
						wall_collisions.push(self.collect_col(self.rb.pos(), self.pos().center(), w_pos));
					}
					self.resolve_col(&wall_collisions);
				}
			}
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
			match sorted_collisions[0].dir {
				Direction::Up=>{
					if self.y_vel() < 0.0 {
						self.set_y_vel(-self.y_vel());
						self.angle = -self.angle;
						self.inc_bounce();
					}
				}
				Direction::Down=>{
					if self.y_vel() > 0.0 {
						self.set_y_vel(-self.y_vel());
						self.angle = -self.angle;
						self.inc_bounce();
					}
				}
				Direction::Right=>{
					if self.x_vel() > 0.0 {
						self.set_x_vel(-self.x_vel());
						self.angle = -self.angle;
						self.inc_bounce();
					}
				}
				Direction::Left=>{
					if self.x_vel() < 0.0 {
						self.set_x_vel(-self.x_vel());
						self.angle = -self.angle;
						self.inc_bounce();
					}
				}
				Direction::None=>{
					println!("I have no clue how this happened");
				}
			}
		}
	}
	
	pub fn update_pos(&mut self) {
		self.set_x(self.x() + self.rb.vel.x as i32);
		self.set_y(self.y() + self.rb.vel.y as i32);

	}
	
	pub fn set_pos(&mut self, p:Rect){
		self.rb.hitbox.x = p.x() as f64;
		self.rb.hitbox.y= p.y() as f64;
	}
	pub fn src(&self) -> Rect{
		return self.src;
	}
	 pub fn die(&mut self){
		// Set death animation when created
		self.is_active = false;
	}

	// actual position 
    pub fn pos(&self) -> Rect {
		return Rect::new(
			self.x() as i32,
			self.y() as i32,
			TILE_SIZE_PROJECTILE, 
			TILE_SIZE_PROJECTILE
		);
    }
	pub fn draw_pos(&self) -> Rect {
		Rect::new(self.x() as i32,
				  self.y() as i32,
				  TILE_SIZE_PROJECTILE, 
				  TILE_SIZE_PROJECTILE)
    }
	// screen coordinates
	pub fn set_cam_pos(&self, player:&Player)-> Rect{
		return Rect::new(
			self.rb.hitbox.x as i32 + (CENTER_W - player.x() as i32),
			self.rb.hitbox.y as i32 + (CENTER_H - player.y() as i32),
			self.rb.hitbox.width(),
			self.rb.hitbox.height()
		);
	}

	pub fn set_cam_pos_large(&self, player:&Player)-> Rect{
		return Rect::new(
			self.rb.hitbox.x as i32 + (CENTER_W - player.x() as i32) - (TILE_SIZE_CAM/2) as i32,
			self.rb.hitbox.y as i32 + (CENTER_H - player.y() as i32) - (TILE_SIZE_CAM/2) as i32,
			TILE_SIZE_PROJECTILE*2,
			TILE_SIZE_PROJECTILE*2
		);
	}

	pub fn inc_bounce(&mut self) {
		self.bounce_counter += 1;
	}

	pub fn get_bounce(&mut self) -> i32 {
		return self.bounce_counter;
	}

	pub fn is_shrapnel(&self) -> bool{return matches!(self.power.power_type,  PowerType::Shrapnel)}

}
