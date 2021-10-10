extern crate rogue_sdl;
use std::time::Duration;
use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use rogue_sdl::SDLCore;
use rogue_sdl::Game;
const TILE_SIZE: u32 = 32;


pub struct Enemy<'a> {
	pos: Rect,
	src: Rect,
	txtre: Texture<'a>,

}

 impl<'a> Enemy<'a> {
	pub fn new(pos: Rect, txtre: Texture<'a>) -> Enemy<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		Enemy {
			pos,
			src,	
			txtre	
		}
	}

	pub fn enemy_x(&self) -> i32 {
		self.pos.x()
	}

	pub fn enemy_y(&self) -> i32 {
		self.pos.y()
	}

	pub fn enemy_width(&self) -> u32 {
		self.pos.width()
	}

	pub fn enemy_height(&self) -> u32 {
		self.pos.height()
	}

	pub fn update_enemy_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	pub fn src(&self) -> Rect {
		self.src
	}

    pub fn txtre(&self) -> &Texture {
        &self.txtre
    }

}