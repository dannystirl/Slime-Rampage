extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;
const TILE_SIZE: u32 = 64;

pub struct Player<'a> {
	delta: Rect, 
	vel: Rect, 
	pos: Rect,
	src: Rect,
	texture_l: Texture<'a>,
    texture_r: Texture<'a>,
	texture_a_l: Texture<'a>,
	texture_a_r: Texture<'a>,
	pub facing_left: bool,
	pub facing_right: bool,
	pub is_still: bool,
	pub hp: f32,
}

impl<'a> Player<'a> {
	pub fn new(pos: Rect, texture_l: Texture<'a>, texture_r: Texture<'a>, texture_a_l: Texture<'a>, texture_a_r: Texture<'a>) -> Player<'a> {
		let delta = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let vel = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let facing_left = false;
		let facing_right = false;
		let is_still = true;
		let mut hp = 100.0;
		Player {
			delta, 
			vel, 
			pos,
			src,
			texture_l,
            texture_r,
			texture_a_l,
			texture_a_r,
            facing_left,
			facing_right,
			is_still,
			hp,
		}
	}

	// player x values
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
	pub fn set_x_delta(&mut self, x:i32){
		self.delta.x = x;
	}
	pub fn x_delta(&self) -> i32 {
		return self.delta.x;
	}
	pub fn width(&self) -> u32 {
		self.pos.width()
	}
	
	// player y values
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
	pub fn set_y_delta(&mut self, y:i32){
		self.delta.y = y;
	}
	pub fn y_delta(&self) -> i32 {
		return self.delta.y;
	}
	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_pos(&mut self, x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_x((self.x() + self.x_vel()).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y((self.y() + self.y_vel()).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn texture_l(&self) -> &Texture {
		&self.texture_l
	}

    pub fn texture_r(&self) -> &Texture {
        &self.texture_r
    }

    pub fn facing_left(&self) -> &bool {
        &self.facing_left
    }

	pub fn facing_right(&self) -> &bool {
        &self.facing_right
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn texture_a_l(&self) -> &Texture {
        &self.texture_a_l
    }

	pub fn texture_a_r(&self) -> &Texture {
        &self.texture_a_r
    }

	pub fn is_still(&self) -> &bool {
        &self.is_still
    }

	pub fn set_src(&mut self, x: i32, y: i32){
		self.src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
	}

	pub fn get_hp(&self) -> f32 {
		return self.hp
	}

	pub fn minus_hp(&mut self, dmg: f32) {
		self.hp -= dmg;
	}
}