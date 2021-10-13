extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::Texture;
const TILE_SIZE: u32 = 64;

pub struct RangedAttack<'a> {
	start_p: Rect, 
	pos: Rect,
	use_ability: bool,
	pub facing_left: bool,
	frame: i32,
    texture: Texture<'a>,
}

 impl<'a> RangedAttack<'a> {
	pub fn new(pos: Rect, use_ability:bool, facing_left: bool, frame:i32, texture: Texture<'a>) -> RangedAttack<'a> {
		let start_p = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
		RangedAttack {
			start_p, 
			pos,	
			use_ability,
			facing_left,
			frame,
			texture, 
		}
	}

	pub fn start_pos(&mut self, x:i32, y:i32, fl:bool) {
		if fl {
			self.facing_left = true;
			self.start_p.x = x-64;
			self.pos.x = x-64;
		}else{
			self.facing_left = false;
			self.start_p.x = x+64;
			self.pos.x = x+64;
		}
		self.start_p.y = y;
		self.pos.y = y;
	}

	// x values
	/*pub fn set_x(&mut self, x:i32){
		self.pos.x = x;
	}*/
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	// y values
	/*pub fn set_y(&mut self, y:i32){
		self.pos.y = y;
	}*/
	/*pub fn y(&self) -> i32 {
		return self.pos.y;
	}*/

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

	// the frames aren't calculating right so the fireball image doesnt look right, but the logic is there. 
	pub fn update_pos(&mut self, x_bounds: (i32, i32)) {
		// form 
		if self.frame<6 {
			self.pos.set_x((self.start_p.x).clamp(x_bounds.0, x_bounds.1));
		// collision
		} else if self.frame>19 {
			self.pos.set_x((self.x()).clamp(x_bounds.0, x_bounds.1));
		// growing / loop 
		} else {
			if self.facing_left {
				self.pos.set_x((self.start_p.x -(self.frame-6)*4 ).clamp(x_bounds.0, x_bounds.1));
			}else{
				self.pos.set_x((self.start_p.x +(self.frame-6)*4 ).clamp(x_bounds.0, x_bounds.1));
			}
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

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
    pub fn pos(&self) -> Rect {
		self.pos
    }
}
