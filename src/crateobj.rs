extern crate rogue_sdl;
use crate::gamedata::*;
use sdl2::rect::Rect;
//use sdl2::image::LoadTexture;
use sdl2::render::{Texture};
use crate::player::*;
use crate::rigidbody::*;
use crate::player::Direction::{Down, Up, Left, Right};
//use crate::rigidbody::*;
use sdl2::rect::Point;

//use sdl2::pixels;
use crate::SDLCore;

pub struct Crate{
	pos: Rect,
	src: Rect,
	vel: (f64,f64),
	velocity: Vec<f64>,
	acceleration: Vec<f64>,
	rb:  Rigidbody,
}


impl Crate {
    pub fn manager() -> Crate{// default constructor also used for manager pretty cool maybe not elegant
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64, TILE_SIZE_64);
        let pos = Rect::new(100 as i32, 100 as i32, TILE_SIZE, TILE_SIZE);
		let vel = (0.0,0.0);
		let velocity = vec![0.0,0.0];
		let acceleration = vec![0.0,0.0];
		let rb = Rigidbody::new(pos); //hitbox

        Crate{
            pos,
            src,
			vel,
			velocity,
			acceleration,
			rb,
        }
    }
	pub fn new(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let vel = (0.0,0.0);
		let velocity = vec![0.0,0.0];
		let acceleration = vec![0.0,0.0];
		let rb = Rigidbody::new(pos);
		Crate{
			pos,
			src,
			vel,
			velocity,
			acceleration,
			rb,
		}
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn set_src(&mut self, new_src: Rect) {
		self.src = new_src;
	}

	pub fn pos(&self) -> Rect {
        self.pos
    }
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}
	pub fn x_vel(&self) -> f64 {
		return self.velocity[0];
	}
	pub fn y_vel(&self) -> f64 {
		return self.velocity[1];
	}
	pub fn update_velocity(&mut self, x: f64, y: f64){
		self.velocity[0] = (self.velocity[0] + x as f64).clamp(-10.0, 10.0);
		self.velocity[1] = (self.velocity[1] + y as f64).clamp(-10.0, 10.0);
	}
	pub fn update_acceleration(&mut self, x: f64, y: f64){
		self.acceleration[0] = x;
		self.acceleration[1] = y;
	}
	pub fn get_magnitude(&self) -> f64{
		return ((self.x_vel() as f64).powf(2.0) + (self.y_vel() as f64).powf(2.0)).sqrt()
	}
	pub fn set_x(&mut self, x: i32){
		self.pos.x = x;
	}
	pub fn set_y(&mut self, y: i32){
		self.pos.y = y;
	}
	pub fn set_x_vel(&mut self, x_vel: f64) {
		self.velocity[0] = x_vel.clamp(-10.0, 10.0);
	}
	pub fn set_y_vel(&mut self, y_vel: f64) {
		self.velocity[1] = y_vel.clamp(-10.0, 10.0);
	}
	pub fn set_rb(&mut self){
		self.rb.set_pos(self.pos);
		self.rb.set_vel(self.vel);

	}
	pub fn update_crates(&mut self, core :&mut SDLCore, crate_textures: &Vec<Texture>, player: &Player, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) {
		// println!("{}, {}", c.velocity[0], c.velocity[1]);
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
					let p_pos = self.pos();//this kind of works?
					if GameData::check_collision(&p_pos, &w_pos) {
						//core.wincan.copy(&crate_textures[0], self.src, debug_pos).unwrap();
						collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
					}
				}
			}
		}
		self.resolve_col(&collisions);
		self.set_x(self.x() + self.velocity[0] as i32);
		self.set_y(self.y() + self.velocity[1] as i32);
		self.set_rb();
		core.wincan.copy(&crate_textures[0],self.src(),self.offset_pos(player)).unwrap();
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
		for c in collisions {
			let new_dir = &c.dir;
			sorted_collisions.push(CollisionDecider::new(*new_dir,c.dist));
		}
		sorted_collisions.sort_by_key(|x| x.dist);

		// Handle collisions based on distance
		if sorted_collisions.len() > 0 {
			match sorted_collisions[0].dir {
				Direction::Up=>{
					self.set_y_vel(self.y_vel().clamp(0.0,100.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir {
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Down=>{
								println!("I have no clue how this happened");
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Down=>{
					self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								println!("I have no clue how this happened");
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Right=>{
					self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Left=>{
								println!("I have no clue how this happened");
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Left=>{
					self.set_x_vel(self.x_vel().clamp(0.0,100.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[1].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
							}
							Direction::Right=>{
								println!("I have no clue how this happened");
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

	pub fn offset_pos(&self, player:&Player)-> Rect{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2, //screen coordinates
					     self.y() as i32 + (CENTER_H - player.y() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
						 TILE_SIZE_PLAYER, TILE_SIZE_PLAYER);
	}
	// restricts movement of crate when not in contact
	pub fn friction(&mut self){
		if self.x_vel() > 0.0 {
			self.update_velocity(-0.1, 0.0);
		} else if self.x_vel() < 0.0 {
			self.update_velocity(0.1, 0.0);
		}
		if self.y_vel() > 0.0 {
			self.update_velocity(0.0, -0.1);
		} else if self.y_vel() < 0.0 {
			self.update_velocity(0.0, 0.1);
		}
	}
	// calculate velocity resistance
	/* fn resist(vel: i32, delta: i32) -> i32 {
		if delta == 0 {
			if vel > 0 {-1}
			else if vel < 0 {1}
			else {delta}
		} else {delta}
	} */
}