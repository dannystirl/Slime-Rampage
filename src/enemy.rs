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
	pub fn set_x(&mut self, x:i32){
		self.pos.x = x;
	}
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	pub fn set_x_vel(&mut self, x:i32){
		self.vel.x = x;
	}
	pub fn x_vel(&self) -> i32 {
		return self.vel.x;
	}
	pub fn width(&self) -> u32 {
		self.pos.width()
	}

	// y values
	pub fn set_y(&mut self, y:i32){
		self.pos.y = y;
	}
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}
	pub fn set_y_vel(&mut self, y:i32){
		self.vel.y = y;
	}
	pub fn y_vel(&self) -> i32 {
		return self.vel.y;
	}
	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_pos(&mut self, roll:i32, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		if roll == 1 {
			self.pos.set_x((self.x() + self.x_vel()+1).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() + self.y_vel()).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 2 {
			self.pos.set_x((self.x() + self.x_vel()).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() + self.y_vel()+1).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 3 {
			self.pos.set_x((self.x() + self.x_vel()).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() + self.y_vel()-1).clamp(y_bounds.0, y_bounds.1));
		}
		if roll == 4 {
			self.pos.set_x((self.x() + self.x_vel()-1).clamp(x_bounds.0, x_bounds.1));
			self.pos.set_y((self.y() + self.y_vel()).clamp(y_bounds.0, y_bounds.1));
		}
	}
	pub fn aggro(&mut self, player_pos_x: i32, player_pos_y:i32){
		let vec = vec![(player_pos_x - self.x()), (player_pos_y-self.y())];
		let mag: i64 = f64::sqrt((vec[0]+vec[1]) as f64)as i64;

		
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
