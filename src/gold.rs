extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;

const TILE_SIZE: u32 = 32;

pub struct Gold<'a>{
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
    amount: u32
}

impl<'a> Gold<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> Gold<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let amount = 1;
		Gold{
			pos,
			src,
			texture,
            amount,
		}
	}


    pub fn src(&self) -> &Rect {
        return &self.src;
    }

    pub fn texture(&self) -> &Texture {
        return &self.texture;
    }

    pub fn pos(&self) -> &Rect {
        return &self.pos;
    }

    pub fn get_gold(&self) -> u32{
        return self.amount;
    }
}