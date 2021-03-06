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
use crate::{power};
use crate::gold::Gold;
use crate::power::*;
use crate::rigidbody::{Rigidbody};

pub enum EnemyType{
	Melee,
	Gellem,
	Skeleton,
	Eyeball,
	Rock,
	Boss,
}
pub struct Enemy<'a> {
	src: Rect,
	angle: f64,
	txtre: Texture<'a>,
	// timers
	stun_timer: Instant,
	fire_timer: Instant,
	damage_timer: Instant,
	// check values
	money: i32,
	has_power: bool,
	pub is_stunned: bool,
	pub x_flipped: bool,
	pub y_flipped: bool,
	pub facing_right: bool,
	pub is_firing: bool,
	pub enemy_type: EnemyType,
	pub enemy_number: usize,
	pub rb: Rigidbody,
	// enemy attributes
	pub alive: bool,
	pub hp: i32,
	pub collision_damage: i32, 
	pub power: Power,
	pub ranged: bool, // enemy has ranged or collision attack
	stun_time: u128, 
	knockback_vel: f64,
	pub speed_delta: f64, 
	pub aggro_range: f64, 
}

 impl<'a> Enemy<'a> {
	pub fn new(pos: Rect, txtre: Texture<'a>, enemy_type: EnemyType, enemy_number: usize, floor_modifier: i32) -> Enemy<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64, TILE_SIZE_64);
		let stun_timer = Instant::now();
		let fire_timer = Instant::now();
		let damage_timer = Instant::now();
		let angle = 0.0;
		let x_flipped = false;
		let has_power = true;
		let y_flipped = false;
		let facing_right = false;
		let is_stunned = false;
		let is_firing =false;
		let alive = true;
		let rb = Rigidbody::new(pos, 0.0, 0.0, 4.0, 0.0);
		
		let hp: i32;
		let stun_time: u128; 
		let knockback_vel: f64; 
		let speed_delta: f64; // multiplicitive value
		let aggro_range: f64; // ~ number of tiles
		let collision_damage: i32; 
		let power: Power; 
		let money: i32; 
		let ranged: bool; 
		match enemy_type {
			EnemyType::Melee => { stun_time = 500; hp = 15 + 10*(floor_modifier-1); knockback_vel = 15.0; 
				speed_delta = 0.5 ; aggro_range = 6.0; collision_damage=5; money=1; ranged = false; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::None); }
			EnemyType::Gellem => { stun_time = 250; hp = 10 + 10*(floor_modifier-1); knockback_vel = 12.0; 
				speed_delta = 0.5 ; aggro_range = 6.0; collision_damage=3; money=2; ranged = true; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Slimeball); }
			EnemyType::Skeleton => { stun_time = 100; hp = 30 + 12*(floor_modifier-1); knockback_vel = 3.0; 
				speed_delta = 0.2 ; aggro_range = 5.0; collision_damage=8; money=3; ranged = false; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Shield); }
			EnemyType::Eyeball => { stun_time = 200; hp = 10 + 7*(floor_modifier-1); knockback_vel = 10.0; 
				speed_delta = 1.0 ; aggro_range = 8.0; collision_damage=3; money=1;  ranged = false; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Dash); }
			EnemyType::Rock => { stun_time = 250; hp = 20 + 15*(floor_modifier-1); knockback_vel = 5.0; 
				speed_delta = 0.3 ; aggro_range = 9.0; collision_damage=3; money=3; ranged = true; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Rock);}
			EnemyType::Boss => { stun_time = 50; hp = 150; knockback_vel = 0.0; 
				speed_delta = 0.3 ; aggro_range = 100.0; collision_damage=10; money=10; ranged = false; 
				power=Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::None);}
		}

		Enemy {
			src,
			txtre,
			stun_time, 
			stun_timer,
			fire_timer,
			damage_timer,
			knockback_vel,
			angle,
			money,
			has_power, 
			x_flipped,
			y_flipped,
			facing_right,
			is_stunned,
			hp,
			collision_damage, 
			power, 
			ranged, 
			alive,
			is_firing,
			enemy_type,
			enemy_number,
			rb,
			speed_delta, 
			aggro_range, 
		}
	}

	// x values
	pub fn set_x(&mut self, x:f64){
		self.rb.hitbox.x = x;
	}
	pub fn x(&self) -> f64 {
		return self.rb.hitbox.x;
	}
	pub fn set_x_vel(&mut self, x:f64){
		self.rb.vel.x = x;
	}
	pub fn x_vel(&self) -> f64 {
		return self.rb.vel.x;
	}
	pub fn width(&self) -> u32 {
		self.rb.hitbox.w as u32
	}

	// y values
	pub fn set_y(&mut self, y:f64){
		self.rb.hitbox.y = y;
	}
	pub fn y(&self) -> f64 {
		return self.rb.hitbox.y;
	}
	pub fn set_y_vel(&mut self, y:f64){
		self.rb.vel.y = y;
	}
	pub fn y_vel(&self) -> f64 {
		return self.rb.vel.y;
	}
	pub fn height(&self) -> u32 {
		self.rb.hitbox.h as u32
	}
	pub fn radius_from_point(&self,(x,y): (f64,f64))->f64{
		let x_d = (self.x() - x).powf(2.0);
		let y_d = (self.y() - y).powf(2.0);
		return (x_d + y_d).sqrt();
	}
	// movement stuff
	pub fn update_pos(&mut self){
		self.rb.hitbox.x = self.rb.hitbox.x + self.rb.vel.x;
		self.rb.hitbox.y = self.rb.hitbox.y + self.rb.vel.y;
	}
	pub fn offset_pos(&self, player:&Player)-> Rect{
		Rect::new(self.rb.hitbox.left() as i32 + (CENTER_W - player.x() as i32),
					self.rb.hitbox.top() as i32 + (CENTER_H - player.y() as i32),
					self.rb.hitbox.width(),
					self.rb.hitbox.height())

	}
	pub fn draw_pos(&self, x: i32, y:i32)-> Rect{
		let r;
		match self.enemy_type {
			EnemyType::Boss => {
				r = Rect::new(
					self.x() as i32 + (CENTER_W - x as i32),
					self.y() as i32 + (CENTER_H - y as i32),
					TILE_SIZE_CAM * 4,
					TILE_SIZE_CAM * 4,
				);
			},
			_ => {
				r = Rect::new(
					self.x() as i32 + (CENTER_W - x as i32),
					self.y() as i32 + (CENTER_H - y as i32),
					TILE_SIZE_CAM,
					TILE_SIZE_CAM,
				);
			}
		}
		return r;
	}
	
	pub fn update_enemy(&mut self, game_data: &GameData, rngt: &Vec<i32>, i: usize, (x,y): (f64,f64), map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Rect {
		// aggro / move
		if self.get_stun_timer() > self.stun_time {
			self.set_stunned(false);
		} 
		
		if self.radius_from_point((x,y)) / TILE_SIZE as f64 > self.aggro_range {
			self.wander(rngt[i]);
		}
		else {
			match self.enemy_type {
				EnemyType::Melee => {
					self.aggro(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
				}
				EnemyType::Gellem => {
					self.flee(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
				}
				EnemyType::Skeleton => {
					self.aggro(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
				}
                EnemyType::Eyeball => {
                    self.aggro(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
				}
				 EnemyType::Rock => {
                    self.flee(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
                }
				EnemyType::Boss => {
					self.aggro(x.into(), y.into(), game_data.get_speed_limit() * self.speed_delta);
				}
			}
		}
		// this should all be copied into force move once its simplified. checking new bounds will solve the bug where enemies will continuously run into a wall.
		let h_bounds_offset = (self.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as f64) as i32;
		let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);
		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {
				let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32 - (CENTER_W - self.x() as i32),
				(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32 - (CENTER_H - self.y() as i32),
				TILE_SIZE, TILE_SIZE);

				let _debug_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32,
				(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32,
				TILE_SIZE, TILE_SIZE);
				if h as i32 + h_bounds_offset < 0 ||
				w as i32 + w_bounds_offset < 0 ||
				h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 ||
							map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 5 {
					let p_pos = self.pos();
					if GameData::check_collision(&p_pos, &w_pos) {
						collisions.push(self.collect_col(p_pos, self.pos().center(), w_pos));
					}
				}
			}
		}
		for c in &game_data.crates{
			  if GameData::check_collision(&self.rb.pos(),&c.pos()){
				collisions.push(self.collect_col(self.rb.pos(), self.rb.hitbox.center_point(), c.pos()));
			}
		}

		self.resolve_col(&collisions);
		self.update_pos();
		if self.is_stunned {
			self.slow_vel(1.0);
		}

		let r;
		match self.enemy_type {
			EnemyType::Boss => {
				r = Rect::new(self.rb.hitbox.left() as i32 + (CENTER_W - x as i32),
				self.rb.hitbox.top() as i32 + (CENTER_H - y as i32),
				self.rb.hitbox.width(),
				self.rb.hitbox.height())
			},
			_ => {
				r = Rect::new(
					self.x() as i32 + (CENTER_W - x as i32),
					self.y() as i32 + (CENTER_H - y as i32),
					TILE_SIZE_CAM,
					TILE_SIZE_CAM,
				);
			}
		}
		return r;
	}

	 pub fn got_squished(&mut self, w_pos: Rect, c_pos: Rect, c_xvel: f64, c_yvel: f64) -> bool{
		 // wall above and crate y_vel negative
		 if self.pos().top() <= w_pos.bottom() && self.pos().bottom() >= c_pos.top() && c_yvel < 0.0{
			 return true;
		 }
		 // wall below and crate y_vel positive
		 else if self.pos().bottom() >= w_pos.top() && self.pos().top() >= c_pos.bottom() && c_yvel > 0.0{
			 return true;
		 }
		 // wall left and crate x_vel negative
		 else if self.pos().left() <= w_pos.right() && self.pos().right() >= c_pos.left() && c_xvel < 0.0{
			 return true;
		 }
		 // wall right and crate x_vel positive
		 else if self.pos().right() >= w_pos.left() && self.pos().left() <= c_pos.right() && c_xvel > 0.0{
			 return true;
		 }
		 else { return false; }
	 }

	pub fn wander(&mut self, roll:i32) {
		if self.is_stunned {
			return;
		}
		if roll == 1 {
			self.set_y_vel(0.0);
			self.set_x_vel(1.0);
			self.facing_right = false;
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
			self.facing_right = true;
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
			self.facing_right = true; 
		}
		else { self.facing_right = false; }
		let mut y = speed_limit_adj * angle.cos();
		if vec[1] < 0.0  {
			y *= -1.0;
		}
		self.set_x_vel(x );
		self.set_y_vel(y );
	}
	
	pub fn flee(&mut self, player_pos_x: f64, player_pos_y: f64, speed_limit_adj: f64) {
		if self.is_stunned {
			return;
		}
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let mut x = speed_limit_adj / 1.5 as f64 * angle.sin();
		if vec[0] >= 0.0 {
			x *= -1.0;
			self.facing_right = false;
		}
		else { self.facing_right = true; }
		let mut y = speed_limit_adj / 1.5 as f64 * angle.cos();
		if vec[1] >= 0.0  {
			y *= -1.0;
		}
		self.set_x_vel(x);
		self.set_y_vel(y);
	}

	pub fn force_move(&mut self, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> bool{
		let h_bounds_offset = (self.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (self.x() / TILE_SIZE as f64) as i32;
		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {
				let w_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32 - (CENTER_W - self.x() as i32),
				(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32 - (CENTER_H - self.y() as i32),
				TILE_SIZE, TILE_SIZE);

				let _debug_pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (self.x() % TILE_SIZE as f64) as i32,
				(h as i32 + 0 as i32) * TILE_SIZE as i32 - (self.y() % TILE_SIZE as f64) as i32,
				TILE_SIZE, TILE_SIZE);
				if h as i32 + h_bounds_offset < 0 ||
				w as i32 + w_bounds_offset < 0 ||
				h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
				w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
				map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
				} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 {
					let p_pos = self.rb.pos();
					if GameData::check_collision(&p_pos, &w_pos) {
						return true; 
					}
				}
			}
		}
		return false; 
	}

	pub fn knockback(&mut self, player_pos_x: f64, player_pos_y: f64) {
		self.x_flipped = false;
		self.y_flipped = false;
		self.is_stunned = true;
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let angle = ((vec[0] / vec[1]).abs()).atan();
		self.angle = angle;
		let mut x = -2.0 * angle.sin();
		if vec[0] < 0.0 {
			x *= -1.0;
			self.x_flipped = true;
		}
		let mut y = -2.0 * angle.cos();
		if vec[1] < 0.0 {
			y *= -1.0;
			self.y_flipped = true;
			self.y_flipped = true;
		}
		self.set_x_vel((self.x_vel() + x).clamp(-self.knockback_vel, self.knockback_vel));
		self.set_y_vel((self.y_vel() + y).clamp(-self.knockback_vel, self.knockback_vel));
		self.stun_timer = Instant::now();
	}

	pub fn projectile_knockback(&mut self, v_x: f64, v_y: f64) {
		self.is_stunned = true;

		self.set_x_vel(self.x_vel() + v_x);
		self.set_y_vel(self.y_vel() + v_y);

		self.stun_timer = Instant::now();
	}

	pub fn src(&self) -> Rect {
		self.src
	}

    pub fn txtre(&self) -> &Texture {
        &self.txtre
    }

    pub fn pos(&self) -> Rect {
        self.rb.pos()
    }

	pub fn get_vel(&self) -> f64 {
		self.knockback_vel
	}

	pub fn slow_vel(&mut self, decel: f64) {
		let mut x_positive = false;
		let mut y_positive = false;

		if self.x_vel() >= 0.0 {
			x_positive = true;
		}
		if self.y_vel() >= 0.0 {
			y_positive = true;
		}

		if x_positive {
			self.set_x_vel(self.x_vel() - decel);
		} else {
			self.set_x_vel(self.x_vel() + decel);
		}
		if y_positive {
			self.set_y_vel(self.y_vel() - decel);
		} else {
			self.set_y_vel(self.y_vel() + decel);
		}
	}

	pub fn angle(&self) -> f64 {
		self.angle
	}

	// attacking
	pub fn check_attack(&mut self, game_data: &mut GameData, (x,y): (f64, f64)) {
		let mut rng = rand::thread_rng();
		if self.ranged {
			if (self.radius_from_point((x,y)) / TILE_SIZE as f64) < self.aggro_range {	// only fire if close enough
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
								self.rb.hitbox.x as i32,
								self.rb.hitbox.y as i32,
								TILE_SIZE_PROJECTILE,
								TILE_SIZE_PROJECTILE,
							),
							true,
							vec![x,y],
							self.power.power_type, 
							0,//elapsed
							0.0
						);
					game_data.enemy_projectiles.push(bullet);
					}
				}
			}
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

	pub fn minus_hp(&mut self, dmg: i32) -> bool {
		if self.damage_timer.elapsed().as_millis() < self.stun_time {
			false
		} else {
			self.damage_timer = Instant::now();
			self.hp -= dmg;
			if self.hp <= 0 {
				self.die();
				match self.enemy_type {
					EnemyType::Boss => {
						if !self.is_alive() { true }
						else { false }
					}
					_ => { false }
				}
			}
			else { false }
		}
	}

	// Set death animation when created
	pub fn die(&mut self){
		self.alive = false;
	}

	pub fn is_alive(&mut self) -> bool{
		return self.alive;
	}

	// items
	pub fn has_money(&self) -> bool {
		return self.money > 0; 
	}

	pub fn money(&self) -> i32 {
		return self.money ; 
	}

	pub fn drop_coin(&mut self) -> Gold {
		let coin = gold::Gold::new(
			Rect::new(
				self.x() as i32 + rand::thread_rng().gen_range(-32..32),
				self.y() as i32 + rand::thread_rng().gen_range(-32..32),
				TILE_SIZE,
				TILE_SIZE,
			),
		);
		self.money -= 1;
		return coin;
	}

	// Powers
	pub fn has_power(&self) -> bool {
		self.has_power
	}

	pub fn drop_power(&mut self) -> Power {
		let mut rng = rand::thread_rng();

		let drop_roll = rng.gen_range(1..4);
		if !self.has_power || drop_roll < 3 {
			self.has_power = false;
			return power::Power::new(
				Rect::new(
					self.x() as i32,
					self.y() as i32,
					TILE_SIZE_POWER,
					TILE_SIZE_POWER,
				),
				PowerType::None,
			);
		}
		let power;
		match self.enemy_type {
			EnemyType::Melee => {
				power = power::Power::new(
					Rect::new(
						self.x() as i32,
						self.y() as i32,
						TILE_SIZE_POWER,
						TILE_SIZE_POWER,
					),
					PowerType::Fireball,
				);
			},
			EnemyType::Gellem => {
				power = power::Power::new(
					Rect::new(
						self.x() as i32,
						self.y() as i32,
						TILE_SIZE_POWER,
						TILE_SIZE_POWER,
					),
					PowerType::Slimeball,
				);
			},
			EnemyType::Skeleton => {
				power = power::Power::new(
					Rect::new(
						self.x() as i32,
						self.y() as i32,
						TILE_SIZE_POWER,
						TILE_SIZE_POWER,
					),
					PowerType::Shield,
				);
			},
			EnemyType::Eyeball => {
				power = power::Power::new(
					Rect::new(
						self.x() as i32,
						self.y() as i32,
						TILE_SIZE,
						TILE_SIZE,
					),
					PowerType::Dash,
				);
			},
			EnemyType::Rock => {
            power = power::Power::new(
                Rect::new(
                    self.x() as i32,
                    self.y() as i32,
                    TILE_SIZE_POWER,
                    TILE_SIZE_POWER,
                ),
                PowerType::Rock,
                );
            },
			EnemyType::Boss => {
				power = power::Power::new(
					Rect::new(
						self.x() as i32,
						self.y() as i32,
						TILE_SIZE,
						TILE_SIZE,
					),
					PowerType::None,
				);
			}
		}
		self.has_power = false;
		return power;
	}

	pub fn has_item(&mut self) -> bool{
		if self.money > 0 || self.has_power {
			return true; 
		}
		return false; 
	}

	// collision
	pub fn collect_col(&mut self, p_pos: Rect, p_center: Point, other_pos :Rect) -> CollisionDecider {
		let distance = ((p_center.x() as f64 - other_pos.center().x() as f64).powf(2.0) + (p_center.y() as f64 - other_pos.center().y() as f64).powf(2.0)).sqrt();

		// enemy above other
		if p_pos.bottom() >= other_pos.top() && p_center.y() < other_pos.top(){
			let resolution = CollisionDecider::new(Down, distance as i32);
			return resolution;
		}
		// enemy left of other
		if p_pos.right() >= other_pos.left() && p_center.x() < other_pos.left() {
			let resolution = CollisionDecider::new(Right, distance as i32);
			return resolution;
		}
		// enemy below other
		if p_pos.top() <= other_pos.bottom() && p_center.y() > other_pos.bottom(){
			let resolution = CollisionDecider::new(Up, distance as i32);
			return resolution;
		}
		// enemy right of other
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
				Direction::Up => {
					self.set_y_vel(self.y_vel().clamp(0.0,100.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir {
							Direction::Up => {
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Left => {
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
							}
							Direction::Right => {
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
							}
							_ => {}
						}
					}
				}
				Direction::Down=>{
					self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir {
							Direction::Down=> {
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Left => {
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
							}
							Direction::Right => {
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
							}
							_ => {}
						}
					}
				}
				Direction::Right => {
					self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[2].dir {
							Direction::Up => {
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Down => {
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Right => {
								self.set_x_vel(self.x_vel().clamp(-100.0,0.0));
							}
							_ => {}
						}
					}
				}
				Direction::Left=>{
					self.set_x_vel(self.x_vel().clamp(0.0,100.0));
					if sorted_collisions.len() > 2 {
						match sorted_collisions[1].dir {
							Direction::Up => {
								self.set_y_vel(self.y_vel().clamp(0.0,100.0));
							}
							Direction::Down => {
								self.set_y_vel(self.y_vel().clamp(-100.0,0.0));
							}
							Direction::Left => {
								self.set_x_vel(self.x_vel().clamp(0.0,100.0));
							}
							_ => {}
						}
					}
				}
				_ => {}
			}
		}
	}
}
