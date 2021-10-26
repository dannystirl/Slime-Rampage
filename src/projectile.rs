extern crate rogue_sdl;

use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

const TILE_SIZE: u32 = 64;
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

pub struct Projectile{
	src: Rect, 
	pos: Rect,
	use_ability: bool,
	pub facing_right: bool,
	frame: i32,
	is_active: bool,
}

 impl Projectile {
	pub fn new(pos: Rect, use_ability:bool, facing_right: bool, frame:i32) -> Projectile {
		let src = Rect::new(0 , 0 , TILE_SIZE, TILE_SIZE);
		let is_active = true;
		Projectile {
			src, 
			pos,	
			use_ability,
			facing_right,
			frame,
			is_active,
		}
	}
	
	/*pub fn start_pos(&mut self, x:i32, y:i32, fr:bool) {
		if fr {
			self.facing_right = true;
			self.start_p.x = x+64;
			self.pos.x = x+64;
		}else{
			self.facing_right = false;
			self.start_p.x = x-64;
			self.pos.x = x-64;
		}
		self.start_p.y = y;
		self.pos.y = y;
	}
*/
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}

	pub fn set_use(&mut self, b:bool){
		self.use_ability = b;
	}
	pub fn is_active(&self) -> bool{
		return self.is_active;
	}

	pub fn set_frame(&mut self, frame:i32){
		self.frame = frame;
	}
	pub fn frame(&self) -> i32 {
		return self.frame;
	}

	// the frames aren't calculating right so the fireball image doesnt look right, but the logic is there. 
	pub fn update_pos(&mut self, player_pos_x: i32, player_pos_y: i32, x_bounds: (i32, i32) ) {
	
		//.self.pos.set_x(self.x() + (CENTER_W - player.x()) + 1).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_x(self.x()-1);
		self.pos.set_y(self.y()+1);

	
	}
	pub fn set_pos(&mut self, p:Rect){
		self.pos = p;
	}
	pub fn src(&self, col: i32, row: i32) -> Rect{
		return Rect::new(
			(self.frame % col) * (TILE_SIZE as i32) * 3/2,
			(self.frame % row) * (TILE_SIZE as i32),
			TILE_SIZE,
			TILE_SIZE,
		);
	}

	 pub fn die(&mut self){
		 // Set death animation when created
		 self.is_active = false;
	 }
    pub fn pos(&self) -> Rect {
		self.pos
    }
}
