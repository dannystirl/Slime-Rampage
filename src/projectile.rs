extern crate rogue_sdl;

use crate::Player;
use sdl2::rect::{Rect, Point};
use crate::gamedata::*;
use crate::SDLCore;
use crate::projectile::Direction::{Down, Up, Left, Right};

#[derive(Copy, Clone)]
pub enum Direction{
	Up,
	Down,
	Left,
	Right,
	None,
}
#[derive(Copy, Clone)]
pub struct CollisionDecider{
	pub dir : Direction,
	pub dist : i32,
}

impl CollisionDecider{
	pub fn new(dir: Direction, dist: i32) -> CollisionDecider{
		let dir = dir;
		let dist = dist;
	CollisionDecider {
		dir,
		dist,
	}

	}
}

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
	/*
	pub fn check_bounce(&mut self, xbounds:(i32,i32), ybounds: (i32,i32)){
		if !DEVELOP {
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

		}
		
	}
	*/
	pub fn update_projectile(&mut self, game_data: &GameData, mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], core: &mut SDLCore) -> Result<(), String>  {
		let tc = core.wincan.texture_creator();
		//let hitbox =tc.load_texture("images/objects/crate.png")?;
		let xwalls = game_data.rooms[0].xwalls;
		let ywalls = game_data.rooms[0].ywalls;
		let speed_limit_adj = game_data.get_speed_limit();
		
		// Slow down to 0 vel if no input and non-zero velocity
		//self.set_x_delta(resist(self.x_vel() as i32, self.x_delta() as i32));
		//self.set_y_delta(resist(self.y_vel() as i32, self.y_delta() as i32));

		// Don't exceed speed limit
		//self.set_x_vel((self.x_vel() + self.x_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
		//self.set_y_vel((self.y_vel() + self.y_delta()).clamp(speed_limit_adj as i32 * -1, speed_limit_adj as i32));
		
		// Stay inside the viewing window
		//self.set_x((self.x() + self.x_vel() as f64));//.clamp(0.0, (xwalls.1 * TILE_SIZE as i32) as f64) as f64);
		//self.set_y((self.y() + self.y_vel() as f64));//.clamp(0.0, (ywalls.1 * TILE_SIZE as i32) as f64) as f64);

		self.set_x(self.x() + self.vector[0] as i32);
		self.set_y(self.y() + self.vector[1] as i32);

		let src = Rect::new(0, 0, TILE_SIZE/4, TILE_SIZE/4);

		let h_bounds_offset = (self.y() as f64/ TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() as f64/ TILE_SIZE as f64) as i32;
		let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);

		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {

			let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() as f64 % TILE_SIZE as f64) as i32 - (CENTER_W - self.x() as i32),
			(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() as f64 % TILE_SIZE as f64) as i32 - (CENTER_H - self.y() as i32),
			TILE_SIZE, TILE_SIZE);

			let debug_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() as f64 % TILE_SIZE as f64) as i32,// - (CENTER_W - self.x() as i32),
			(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() as f64 % TILE_SIZE as f64) as i32,// - (CENTER_H - self.y() as i32),
			TILE_SIZE, TILE_SIZE);
			if h as i32 + h_bounds_offset < 0 ||
		  	 w as i32 + w_bounds_offset < 0 ||
		  	 h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
		  	 w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
		   	map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
			continue;
			} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 {
				let p_pos = self.pos();

				//core.wincan.copy(&hitbox, src, w_pos);
				if GameData::check_collision(&p_pos, &w_pos) {
				//	core.wincan.copy(&hitbox, src, self.cam_pos);
				//	core.wincan.copy(&hitbox, src, debug_pos);
					collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
				}
			}
			}
		}
		
		self.resolve_col(&collisions);

		for c in &game_data.crates{
			let crate_pos = c.pos();
			let p_pos =self.pos();
			if GameData::check_collision(&self.pos(), &c.pos()) {//I hate collisions
				//println!("welcome to hell");
				self.collect_col(self.pos(), self.pos().center(), c.pos());
			}
		}
		//self.update_pos((-100 * TILE_SIZE as i32, 100 * TILE_SIZE as i32), (-100 * TILE_SIZE as i32, 100 * TILE_SIZE as i32));/* game_data.rooms[0].xbounds, game_data.rooms[0].ybounds */
		self.update_pos();
		
		
		
		Ok(())
	}

	pub fn update_pos(&mut self) {
		self.set_x(self.x() + self.vector[0] as i32);
		self.set_y(self.y() + self.vector[1] as i32);
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

	pub fn resolve_col(&mut self, collisions : &Vec<CollisionDecider>) {
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
					self.set_y_vel(self.y_vel().clamp(0.0,100.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
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
	

