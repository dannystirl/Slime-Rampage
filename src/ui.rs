extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct UI<'a>{
	pos: Rect,
	pub texture: Texture<'a>,
	//src: Rect,
}

impl<'a> UI<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> UI<'a> {
		//let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);

		UI{
			pos,
			texture,
			//src,
		}
	
	}

	pub fn pos(&self) -> Rect {
        self.pos
    }

	pub fn texture(&self) -> &Texture {
        &self.texture
    }

}