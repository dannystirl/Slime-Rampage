extern crate rogue_sdl;

use std::time::Instant;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE * 3 / 2;
const COOLDOWN: u128 = 250;
const DMG_COOLDOWN: u128 = 800;

pub struct Player<'a> {
	pos: (i32, i32),
	cam_pos: Rect,
	vel: (i32, i32), 
	delta: (i32, i32), 
	height: u32,
	width: u32,
	src: Rect,
	attack_box: Rect,
	attack_timer: Instant,
	damage_timer: Instant,
	texture_all: Texture<'a>,
	invincible: bool, 
	pub facing_right: bool,
	pub is_still: bool,
	pub hp: f32,
	pub is_attacking: bool,
	
}

impl<'a> Player<'a> {
	pub fn new(pos: (i32,i32), texture_all: Texture<'a>) -> Player<'a> {
		let cam_pos = Rect::new(
			0,
			0,
			TILE_SIZE,
			TILE_SIZE,
		);
		let vel = (0,0);
		let delta = (0, 0);
		let height = TILE_SIZE;//32;
		let width = TILE_SIZE;//32;
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let hp = 100.0;
		let facing_right = false;
		let is_still = true;
		let is_attacking = false;
		let attack_box = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let attack_timer = Instant::now();
		let damage_timer = Instant::now();
		let invincible = true;
		Player {
			pos,
			cam_pos,
			vel, 
			delta, 
			height,
			width,
			src,
			attack_box,
			attack_timer,
			damage_timer,
			invincible, 
			texture_all,
			facing_right,
			is_still,
			hp,
			is_attacking,
		}
	}

	// player x values
	pub fn set_x(&mut self, x:i32){
		self.pos.0 = x;
	}
	pub fn x(&self) -> i32 {
		return self.pos.0;
	}
	pub fn set_x_vel(&mut self, x:i32){
		self.vel.0 = x;
	}
	pub fn x_vel(&self) -> i32 {
		return self.vel.0;
	}
	pub fn set_x_delta(&mut self, x:i32){
		self.delta.0 = x;
	}
	pub fn x_delta(&self) -> i32 {
		return self.delta.0;
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	
	// player y values
	pub fn set_y(&mut self, y:i32){
		self.pos.1 = y;
	}
	pub fn y(&self) -> i32 {
		return self.pos.1;
	}
	pub fn set_y_vel(&mut self, y:i32){
		self.vel.1 = y;
	}
	pub fn y_vel(&self) -> i32 {
		return self.vel.1;
	}
	pub fn set_y_delta(&mut self, y:i32){
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
		self.pos.0 = (self.x() + self.x_vel()).clamp(x_bounds.0, x_bounds.1);
		self.pos.1 = (self.y() + self.y_vel()).clamp(y_bounds.0, y_bounds.1);
	}

	pub fn set_src(&mut self, x: i32, y: i32) {
		self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
	}

	pub fn src(&self) -> Rect {
		self.src
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

	pub fn pos(&self) -> Rect {
        return Rect::new(
			self.x(),
			self.y(),
			TILE_SIZE,
			TILE_SIZE,
		)
    }

	pub fn set_cam_pos(&mut self, x:i32, y:i32) {
		self.cam_pos = Rect::new(
			self.x() - x,
			self.y() - y,
			TILE_SIZE,
			TILE_SIZE,
		);
	}
	
	pub fn get_cam_pos(&self) -> Rect {
        self.cam_pos
    }

	pub fn texture_all(&self) -> &Texture {
        &self.texture_all
    }

	pub fn is_still(&self) -> &bool {
        &self.is_still
    }

	// attacking values
	pub fn get_attack_timer(&self) -> u128 {
		self.attack_timer.elapsed().as_millis()
	}

	pub fn get_damage_timer(&self) -> u128 {
		self.damage_timer.elapsed().as_millis()
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
		if self.get_attack_timer() < COOLDOWN {
			return;
		}
		self.is_attacking = true;
		self.set_attack_box(self.x(), self.y());
		self.attack_timer = Instant::now();
	}

	pub fn clear_attack_box(&mut self) {
		self.attack_box = Rect::new(self.x() as i32, self.y() as i32, 0, 0);
	}

	pub fn set_cooldown(&mut self) {
		self.is_attacking = false;
		self.clear_attack_box();
	}

	pub fn get_cooldown(&self) -> u128 {
		COOLDOWN
	}

	// heatlh values
	pub fn get_hp(&self) -> f32 {
		return self.hp
	}

	pub fn is_dead(&self) -> bool {
		return self.hp <= 0.0;
	}

	pub fn minus_hp(&mut self, dmg: f32) {
		if self.invincible {
			return;
		}
		self.hp -= dmg;
		self.damage_timer = Instant::now();
	}

	pub fn set_invincible(&mut self){
		if self.get_damage_timer() < DMG_COOLDOWN {
			 self.invincible = true; 
		} else {
			self.invincible = false;
		}
	}

	pub fn get_invincible(&self) -> bool {
		self.invincible
	}
	
	pub fn display_weapon(&self){
	
	}
}
