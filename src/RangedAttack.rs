extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;

const TILE_SIZE: u32 = 64;

pub struct RangedAttack<'a> {
	startP: Rect, 
	pos: Rect,
	src: Rect,
	txtre: Texture<'a>,
	use_ability: bool,
	frame: i32,
}

 impl<'a> RangedAttack<'a> {
	pub fn new(startP: Rect, pos: Rect, txtre: Texture<'a>, use_ability:bool, frame:i32) -> RangedAttack<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		RangedAttack {
			startP, 
			pos,
			src,	
			txtre,
			use_ability,
			frame,
		}
	}

	pub fn start_pos(&mut self, x:i32, y:i32) {
		self.startP.x = x;
		self.pos.x = x;
		self.startP.y = y;
		self.pos.y = y;
	}

	// x values
	pub fn set_x(&mut self, x:i32){
		self.pos.x = x;
	}
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	// y values
	pub fn set_y(&mut self, y:i32){
		self.pos.y = y;
	}
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}

	pub fn set_use(&mut self, b:bool){
		self.use_ability = b;
	}
	pub fn in_use(&self) -> bool{
		return self.use_ability;
	}

	pub fn set_frame(&mut self, frame:i32){
		self.frame = frame;
	}
	pub fn frame(&self) -> i32 {
		return self.frame;
	}

	pub fn update_RangedAttack_pos(&mut self, x_bounds: (i32, i32)) {
		if self.frame<6 {
			self.pos.set_x((self.startP.x).clamp(x_bounds.0, x_bounds.1));
		} else {
			self.pos.set_x((self.startP.x +(self.frame-6)*16 ).clamp(x_bounds.0, x_bounds.1));
		}
	}

	pub fn src(&self, col: i32, row: i32) -> Rect{
		return Rect::new(
			(self.frame % col) * (TILE_SIZE as i32) * 3/2,
			(self.frame % row) * (TILE_SIZE as i32),
			TILE_SIZE,
			TILE_SIZE,
		);
	}

    pub fn txtre(&self) -> &Texture {
        &self.txtre
    }
    pub fn pos(&self) -> Rect {
		self.pos
    }
}