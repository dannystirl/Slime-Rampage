extern crate rogue_sdl;
use crate::gamedata::*;

use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct UI<'a>{
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
}

impl<'a> UI<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> UI<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		UI{
			pos,
			src,
			texture,
		}
	}

	pub fn src(&self) -> Rect {
		self.src
	}

	pub fn set_src(&mut self, new_src: Rect) {
		self.src = new_src;
	}

	pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn texture(&self) -> &Texture {
        &self.texture
    }

}