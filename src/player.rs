extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;
const TILE_SIZE: u32 = 64;

pub struct Player<'a> {
	pos: Rect,
	src: Rect,
	texture_l: Texture<'a>,
    texture_r: Texture<'a>,
    pub facing_left: bool,
}

impl<'a> Player<'a> {
	pub fn new(pos: Rect, texture_l: Texture<'a>, texture_r: Texture<'a>) -> Player<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let facing_left = false;
		Player {
			pos,
			src,
			texture_l,
            texture_r,
            facing_left,
		}
	}

	pub fn x(&self) -> i32 {
		self.pos.x()
	}

	pub fn y(&self) -> i32 {
		self.pos.y()
	}

	pub fn width(&self) -> u32 {
		self.pos.width()
	}

	pub fn height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
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
    pub fn pos(&self) -> Rect {
        self.pos
    }
}