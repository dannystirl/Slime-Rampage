extern crate rogue_sdl;
use crate::gamedata::*;
use crate::vector::Vector2D;
use sdl2::rect::Rect;
use sdl2::render::{Texture};
use crate::player::*;
use crate::rigidbody::*;
use crate::player::Direction::{Down, Up, Left, Right};
//use crate::rigidbody::*;
use sdl2::rect::Point;
use crate::projectile;
use crate::projectile::*;

//use sdl2::pixels;
use crate::SDLCore;

pub const MAX_CRATE_SPEED: f64 = 2.0; 
pub const MAX_CRATE_VEL: f64 = 6.0;
pub const EXPLODE_SPEED: f64 = 6.0;

pub struct Crate{
	src: Rect,
	pub rb:  Rigidbody,
	heavy: bool,
	explosive: bool,
	active: bool,
}
//impl Explosive for Crate{

//}
impl Crate {
	pub fn new(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64, TILE_SIZE_64);
		let rb = Rigidbody::new(pos, 0.0, 0.0,1.0, 0.05);
		let heavy= false;
		let explosive = false;
		let active = true;
		Crate{
			src,
			rb,
			heavy,
			explosive,
			active,
		}
	}
	pub fn new_heavy(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64, TILE_SIZE_64);
		let rb = Rigidbody::new(pos, 0.0, 0.0, 5.0, 0.1);
		let heavy = true;
		let explosive = false;
		let active = true;
		Crate{
			src,
			rb,
			heavy,
			explosive,
			active,
		}
	}
	pub fn new_explosive(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64, TILE_SIZE_64);
		let rb = Rigidbody::new(pos, 0.0, 0.0, 5.0, 0.1);
		let heavy = false;
		let explosive = true;
		let active = true;
		Crate{
			src,
			rb,
			heavy,
			explosive,
			active,
		}
	}

	pub fn is_active(&self) -> bool{self.active}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn set_src(&mut self, new_src: Rect) {
		self.src = new_src;
	}

	pub fn pos(&self) -> Rect {
        self.rb.draw_pos()
    }
	pub fn x(&self) -> i32 {
		return self.rb.hitbox.x as i32;
	}
	pub fn y(&self) -> i32 {
		return self.rb.hitbox.y as i32;
	}
	pub fn x_vel(&self) -> f64 {
		return self.rb.vel.x;
	}
	pub fn y_vel(&self) -> f64 {
		return self.rb.vel.y;
	}
	pub fn update_velocity(&mut self, x: f64, y: f64){
		self.rb.vel.x = (self.rb.vel.x + x as f64).clamp(-MAX_CRATE_VEL, MAX_CRATE_VEL);
		self.rb.vel.y = (self.rb.vel.y + y as f64).clamp(-MAX_CRATE_VEL, MAX_CRATE_VEL);
	}
	pub fn update_acceleration(&mut self, x: f64, y: f64){
		// self.acceleration[0] = x;
		// self.acceleration[1] = y;
	}
	pub fn get_magnitude(&self) -> f64{
		return ((self.x_vel() as f64).powf(2.0) + (self.y_vel() as f64).powf(2.0)).sqrt()
	}
	pub fn set_x(&mut self, x: i32){
		self.rb.hitbox.x = x as f64;
	}
	pub fn set_y(&mut self, y: i32){
		self.rb.hitbox.y = y as f64;
	}
	pub fn set_x_vel(&mut self, x_vel: f64) {
		self.rb.vel.x = x_vel.clamp(-MAX_CRATE_VEL, MAX_CRATE_VEL);
	}
	pub fn set_y_vel(&mut self, y_vel: f64) {
		self.rb.vel.y = y_vel.clamp(-MAX_CRATE_VEL, MAX_CRATE_VEL);
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
									  TILE_SIZE_CAM, TILE_SIZE_CAM);

				if h as i32 + h_bounds_offset < 0 ||
				w as i32 + w_bounds_offset < 0 ||
				h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 ||
					map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 5  {
					let p_pos = self.rb.draw_pos();//this kind of works?
					let normal_collision = &mut Vector2D{x : 0.0, y : 0.0};
					let pen = &mut 0.0;
					let mut wall = Rigidbody::new_static(w_pos, 0.0,0.0, 2.0);
					if wall.rect_vs_rect(self.rb, normal_collision,  pen){
						wall.resolve_col(&mut self.rb, *normal_collision, *pen);
					}
					if GameData::check_collision(&p_pos, &w_pos) {
						//core.wincan.copy(&crate_textures[0], self.src, debug_pos).unwrap();
						collisions.push(self.collect_col(p_pos, self.rb.hitbox.center_point(), w_pos));
					}
				}
			}
		}
		self.resolve_col(&collisions);
		self.set_x(self.x() + self.rb.vel.x as i32);
		self.set_y(self.y() + self.rb.vel.y as i32);
		if self.heavy{
			core.wincan.copy(&crate_textures[1],self.src(),self.offset_pos(player)).unwrap();
		}else if self.explosive{
			core.wincan.copy(&crate_textures[2],self.src(),self.offset_pos(player)).unwrap();
		}else{
		core.wincan.copy(&crate_textures[0],self.src(),self.offset_pos(player)).unwrap();
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
		for c in collisions {
			let new_dir = &c.dir;
			sorted_collisions.push(CollisionDecider::new(*new_dir,c.dist));
		}
		sorted_collisions.sort_by_key(|x| x.dist);

		// Handle collisions based on distance
		if sorted_collisions.len() > 0 {
			match sorted_collisions[0].dir {
				Direction::Up=>{
					self.set_y_vel(self.y_vel().clamp(0.0,MAX_CRATE_SPEED));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir {
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,MAX_CRATE_SPEED));
							}
							Direction::Down=>{
								println!("I have no clue how this happened");
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,MAX_CRATE_SPEED));
	
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-MAX_CRATE_SPEED,0.0));
	
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Down=>{
					self.set_y_vel(self.y_vel().clamp(-MAX_CRATE_SPEED,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								println!("I have no clue how this happened");
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-MAX_CRATE_SPEED,0.0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,MAX_CRATE_SPEED));
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-MAX_CRATE_SPEED,0.0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Right=>{
					self.set_x_vel(self.x_vel().clamp(-MAX_CRATE_SPEED,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,MAX_CRATE_SPEED));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-MAX_CRATE_SPEED,0.0));
							}
							Direction::Left=>{
								println!("I have no clue how this happened");
							}
							Direction::Right=>{
								self.set_x_vel(self.x_vel().clamp(-MAX_CRATE_SPEED,0.0));
							}
							Direction::None=>{
								println!("I have no clue how this happened");
							}
						}
					}
				}
				Direction::Left=>{
					self.set_x_vel(self.x_vel().clamp(0.0,MAX_CRATE_SPEED));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[1].dir{
							Direction::Up=>{
								self.set_y_vel(self.y_vel().clamp(0.0,MAX_CRATE_SPEED));
							}
							Direction::Down=>{
								self.set_y_vel(self.y_vel().clamp(-MAX_CRATE_SPEED,0.0));
							}
							Direction::Left=>{
								self.set_x_vel(self.x_vel().clamp(0.0,MAX_CRATE_SPEED));
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
		if self.explosive{
			return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2, //screen coordinates
							 self.y() as i32 + (CENTER_H - player.y() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
							 TILE_SIZE_PLAYER*2, TILE_SIZE_PLAYER*2);
		}
		else if self.heavy{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2, //screen coordinates
					     self.y() as i32 + (CENTER_H - player.y() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
						 TILE_SIZE_PLAYER*2, TILE_SIZE_PLAYER*2);
		}
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2, //screen coordinates
					     self.y() as i32 + (CENTER_H - player.y() as i32) + (TILE_SIZE_CAM as i32 - TILE_SIZE_PLAYER as i32).abs()/2,
						 TILE_SIZE_PLAYER, TILE_SIZE_PLAYER);
	}
	// restricts movement of crate when not in contact
	pub fn friction(&mut self){
		if self.x_vel() > 0.0 {
			self.update_velocity(-self.rb.friction, 0.0);
		} else if self.x_vel() < 0.0 {
			self.update_velocity(self.rb.friction, 0.0);
		}
		if self.y_vel() > 0.0 {
			self.update_velocity(0.0, -self.rb.friction);
		} else if self.y_vel() < 0.0 {
			self.update_velocity(0.0, self.rb.friction);
		}
	}

	// Method for explosive crate.
	pub fn explode(&mut self, elapsed: u128) -> Vec<Projectile>{
		self.active = false;
		let mut shrapnel: Vec<Projectile> = Vec::with_capacity(69);
		for i in 0..8 {
			// N
			if i == 0 {
				let scrap = projectile::Projectile::new(
					Rect::new(self.rb.hitbox.x as i32, (self.rb.hitbox.y - 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![0.0, -EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// NE
			if i == 1 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x + 40.0) as i32, (self.rb.hitbox.y - 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![EXPLODE_SPEED, -EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// E
			if i == 2 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x + 40.0) as i32, self.rb.hitbox.y as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![EXPLODE_SPEED, 0.0],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// SE
			if i == 3 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x + 40.0) as i32, (self.rb.hitbox.y + 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![EXPLODE_SPEED, EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// S
			if i == 4 {
				let scrap = projectile::Projectile::new(
					Rect::new(self.rb.hitbox.x as i32, (self.rb.hitbox.y + 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![0.0, EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// SW
			if i == 5 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x - 40.0) as i32, (self.rb.hitbox.y + 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![-EXPLODE_SPEED, EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// W
			if i == 6 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x - 40.0) as i32, self.rb.hitbox.y as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![-EXPLODE_SPEED, 0.0],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
			// NW
			if i == 7 {
				let scrap = projectile::Projectile::new(
					Rect::new((self.rb.hitbox.x - 40.0) as i32, (self.rb.hitbox.y - 40.0) as i32,
							  TILE_SIZE_PROJECTILE, TILE_SIZE_PROJECTILE, ),
					false,
					vec![-EXPLODE_SPEED, -EXPLODE_SPEED],
					ProjectileType::Bullet,
					elapsed,
				);
				shrapnel.push(scrap);
			}
		}
		return shrapnel
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