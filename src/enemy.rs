extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::f64;

const TILE_SIZE: u32 = 64;

pub struct Enemy<'a> {
	vel: Rect, 
	pos: Rect,
	src: Rect,
	txtre: Texture<'a>,
	pub facing_left: bool,
	pub hp: f32,
	pub alive: bool,
}

 impl<'a> Enemy<'a> {
	pub fn new(pos: Rect, txtre: Texture<'a>) -> Enemy<'a> {
		let vel = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let facing_left = false;
		let hp = 50.0;
		let alive = true;
		Enemy {
			vel, 
			pos,
			src,	
			txtre,
			facing_left,
			hp,
			alive,
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

	pub fn update_pos(&mut self, roll:i32, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
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

	pub fn aggro(&mut self, player_pos_x: f64, player_pos_y: f64, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		let vec = vec![player_pos_x - self.x(), player_pos_y - self.y()];
		let speed: f64 = 3.0;
		let angle = ((vec[0] / vec[1]).abs()).atan();
		let mut x = speed * angle.sin();
		if(vec[0] < 0.0) {
			x *= -1.0;
		}
		let mut y = speed * angle.cos();
		if(vec[1] < 0.0) {
			y *= -1.0;
		}
		self.pos.set_x(((self.x() + x) as i32).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y(((self.y() + y) as i32).clamp(y_bounds.0, y_bounds.1));
		
		//println!("{}", mag);
	}

	pub fn src(&self) -> Rect {
		self.src
	}

    pub fn txtre(&self) -> &Texture {
        &self.txtre
    }

	pub fn facing_left(&self) -> &bool {
        &self.facing_left
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

	 pub fn minus_hp(&mut self, dmg: f32) {
		 self.hp -= dmg;

		 if self.hp <= 0.0{
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
}
