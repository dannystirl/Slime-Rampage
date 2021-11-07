extern crate rogue_sdl;
use crate::gamedata::GameData;
use crate::gamedata::*;
use crate::projectile::*;
use crate::player::*;
use crate::player::Direction::{Down, Up, Left, Right};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Instant;
use sdl2::render::Texture;
use rand::Rng;
use crate::{gold};
//use rogue_sdl::{Game, SDLCore};
use crate::gold::Gold;
pub enum EnemyType{
	Melee,
	Ranged,
}
pub struct Enemy<'a> {
	vel: Rect,
	pos: Rect,
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
	pub enemy_type: EnemyType,
	pub enemy_number: usize,
}

 impl<'a> Enemy<'a> {
	pub fn new(pos: Rect, txtre: Texture<'a>, enemy_type: EnemyType, num: usize) -> Enemy<'a> {
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
		let enemy_number = num;
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
	pub fn radius_from_point(&self,(x,y): (f64,f64))->f64{
		let x_d = (self.x() - x).powf(2.0);
		let y_d = (self.y() - y).powf(2.0);
		return (x_d + y_d).sqrt();
	}
	// movement stuff
	pub fn update_pos(&mut self){
		self.pos.set_x(self.x() as i32 +self.x_vel() as i32);
		self.pos.set_y(self.y() as i32 + self.y_vel() as i32);
	}
	#[allow(unused_parens)]
	pub fn update_enemy(&mut self, game_data: &GameData, rngt: &Vec<i32>, i: usize, (x,y): (f64,f64), map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Rect {
	
		// aggro / move
		let distance = self.radius_from_point((x,y));
		if self.get_stun_timer() > 1000 {
			self.set_stunned(false);
		} 
		if distance > 300.0 {
			self.wander(rngt[i]/* , xbounds, ybounds*/);
		} else {
			match self.enemy_type {
			EnemyType::Melee=>{
				self.aggro(x.into(), y.into(), game_data.get_speed_limit());}
			EnemyType::Ranged =>{
				self.flee(x.into(), y.into(), game_data.get_speed_limit());
			}
		}
	}
		let h_bounds_offset = (self.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as f64) as i32;
		let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);
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

					if GameData::check_collision(&p_pos, &w_pos) {
						collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
					}
				}
			}
		}
		self.resolve_col(&collisions);

	self.update_pos();
		return Rect::new(self.x() as i32 + (CENTER_W - x as i32),
						 self.y() as i32 + (CENTER_H - y as i32),
						 TILE_SIZE / 2, TILE_SIZE / 2);
	}

	#[allow(unused_parens)]
	pub fn wander(&mut self, roll:i32) {

		if self.is_stunned {
			return;
		}
		if roll == 1 {
			self.set_y_vel(0.0);
			self.set_x_vel(1.0);
		}
		if roll == 2 {
			self.set_y_vel(-1.0);
			self.set_x_vel(0.0);

		}
		if roll == 3 {
			self.set_y_vel(1.0);
			self.set_x_vel(0.0);

		}
		if roll == 4 {
			self.set_x_vel(-1.0);
			self.set_y_vel(0.0);

		}
	}

	#[allow(unused_parens)]
	pub fn aggro(&mut self, player_pos_x: f64, player_pos_y: f64,speed_limit_adj: f64) {
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		if self.is_stunned || ((vec[0].abs() < 0.1) && (vec[1].abs() < 0.1)) {
			return;
		}
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let mut x = speed_limit_adj * angle.sin();
		if vec[0] < 0.0 {
			x *= -1.0;
		}
		let mut y = speed_limit_adj * angle.cos();
		if vec[1] < 0.0  {
			y *= -1.0;
		}
		self.set_x_vel(x );
		self.set_y_vel(y );
	}

	pub fn flee(&mut self, player_pos_x: f64, player_pos_y: f64, /* x_bounds: (i32, i32), y_bounds: (i32, i32),  */speed_limit_adj: f64) {
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
		self.set_x_vel(x);
		self.set_y_vel(y);
	}

	pub fn force_move(&mut self, game_data: &GameData) -> bool{
		let xbounds = game_data.rooms[game_data.current_room].xbounds;
		let ybounds = game_data.rooms[game_data.current_room].ybounds;
		if  self.x() <= xbounds.0 as f64 ||
		self.x() >=  xbounds.1 as f64 ||
		self.y() <= ybounds.0 as f64||
		self.y() >= ybounds.1 as f64
		{return true;}
		else {return false;}
	}

	pub fn knockback(&mut self, player_pos_x: f64, player_pos_y: f64, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
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
	pub fn check_attack(&mut self, game_data: &mut GameData, (x,y): (f64, f64)) {
		let mut rng = rand::thread_rng();
		match self.enemy_type {
			EnemyType::Ranged=>{
				if self.get_fire_timer() > self.get_fire_cooldown() {
					self.set_fire_cooldown();
					let fire_chance = rng.gen_range(1..60);
					if fire_chance < 5 { // chance to fire
						self.fire(); // sets is firing true
						let vec = vec![x - self.x(), y - self.y()];
						let angle = ((vec[0] / vec[1]).abs()).atan();
						let mut x = &game_data.get_speed_limit() * angle.sin();
						let mut y = &game_data.get_speed_limit() * angle.cos();
						if vec[0] < 0.0 {
							x *= -1.0;
						}
						if vec[1] < 0.0  {
							y *= -1.0;
						}
						let bullet = Projectile::new(
							Rect::new(
								self.pos().x(),
								self.pos().y(),
								TILE_SIZE/2,
								TILE_SIZE/2,
							),
							true,
							vec![x,y],
							ProjectileType::Bullet,
						);
					game_data.enemy_projectiles.push(bullet);
					}
				}
			} EnemyType::Melee=>{}
		}
	}

	pub fn get_fire_timer(&self) -> u128 {
		self.fire_timer.elapsed().as_millis()
	}

	pub fn fire(&mut self){
		if self.get_fire_timer() < FIRE_COOLDOWN_E {
		 return;
		}
		self.is_firing = true;
		self.fire_timer = Instant::now();

	}

	pub fn get_fire_cooldown(&self)-> u128{
		FIRE_COOLDOWN_E
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

	 pub fn drop_item(&mut self) -> Gold {
		 let coin = gold::Gold::new(
			 Rect::new(
				 self.x() as i32,
				 self.y() as i32,
				 TILE_SIZE,
				 TILE_SIZE,
			 ),
		 );
		 self.set_no_gold();
		 return coin;
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
}
