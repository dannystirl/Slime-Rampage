extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;

const TILE_SIZE: u32 = 64;

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

	pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn texture(&self) -> &Texture {
        &self.texture
    }

}