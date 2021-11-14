extern crate rogue_sdl;

use sdl2::rect::Rect;
//use crate::gamedata::*;
//use sdl2::render::Texture;
use crate::gamedata::*;
use crate::Player;

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
    pub fn offset_pos(&self, player:&Player)-> Rect{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
							self.y() as i32 + (CENTER_H - player.y() as i32),
		TILE_SIZE, TILE_SIZE);
	}
}