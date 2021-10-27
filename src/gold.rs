extern crate rogue_sdl;

use sdl2::rect::Rect;
//use sdl2::render::Texture;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;
const TILE_SIZE: u32 = 64;

pub struct Gold{
	pos: Rect,
	src: Rect,
    amount: u32,
    been_collected: bool,
}

impl Gold {
	pub fn new(pos: Rect) -> Gold {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let amount = 1;
        let been_collected = false;
		Gold{
			pos,
			src,
            amount,
            been_collected,
		}
	}

    pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}

    pub fn src(&self) -> Rect {
        self.src
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

    pub fn get_gold(&self) -> u32{
        return self.amount;
    }

    pub fn collected(&self) -> bool {
        return self.been_collected;
    }

    pub fn set_collected(&mut self)  {
        self.been_collected = true;
    }
}