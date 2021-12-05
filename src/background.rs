extern crate rogue_sdl;

use sdl2::render::Texture;
use sdl2::rect::Rect;
use crate::gamedata::*;

pub struct Background<'a> {
	pub black: Texture<'a>,
	pub texture_0: Texture<'a>,
	pub texture_1: Texture<'a>,
	pub texture_2: Texture<'a>,
	pub texture_3: Texture<'a>,
	pub upstairs: Texture<'a>,
	pub downstairs: Texture<'a>,
	pub dirt_sheet: Texture<'a>,
	curr_bg: Rect, 
}

impl<'a> Background<'a> {
	pub fn new(black: Texture<'a>, texture_0: Texture<'a>, texture_1: Texture<'a>, texture_2: Texture<'a>, texture_3: Texture<'a>, upstairs: Texture<'a>, downstairs: Texture<'a>, dirt_sheet: Texture<'a>, 
			   curr_bg: Rect) -> Background<'a> {
		Background {
			black,
			texture_0, 
			texture_1, 
			texture_2, 
			texture_3,
			upstairs,
			downstairs,
			dirt_sheet, 
			curr_bg, 
		}
	}

	pub fn set_curr_background(&mut self, x:f64, y:f64, w: u32, h:u32){
		self.curr_bg = Rect::new(
			(x as i32 + ((w / 2) as i32)) - ((CAM_W / 2) as i32),
			(y as i32 + ((h / 2) as i32)) - ((CAM_H / 2) as i32),
			CAM_W,
			CAM_H,
		);
	}

	pub fn get_curr_background(&self) -> Rect {
		self.curr_bg
	}

	pub fn texture(&self) -> &Texture {
        &self.texture_0
    }
}