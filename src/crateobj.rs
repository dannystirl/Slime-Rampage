extern crate rogue_sdl;
use crate::gamedata::*;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use crate::player::*;
use sdl2::pixels;
use crate::SDLCore;

pub struct Crate{
	pos: Rect,
	src: Rect,
	velocity: Vec<f64>,
	acceleration: Vec<f64>,
}

impl Crate {
    pub fn manager() -> Crate{// default constructor also used for manager pretty cool maybe not elegant
        let pos = Rect::new(100 as i32, 100 as i32, TILE_SIZE, TILE_SIZE);
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let velocity = vec![0.0,0.0];
		let acceleration = vec![0.0,0.0];
        Crate{
            pos,
            src,
			velocity,
			acceleration
        }
    }
	pub fn new(pos: Rect) -> Crate {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		let velocity = vec![0.0,0.0];
		let acceleration = vec![0.0,0.0];
		Crate{
			pos,
			src,
			velocity,
			acceleration,
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
	pub fn x(&self) -> i32 {
		return self.pos.x;
	}
	pub fn y(&self) -> i32 {
		return self.pos.y;
	}
	pub fn x_vel(&self) -> f64 {
		return self.velocity[0];
	}
	pub fn y_vel(&self) -> f64 {
		return self.velocity[1];
	}
	pub fn update_velocity(&mut self, x: f64, y: f64){
		self.velocity[0] = self.velocity[0] + x as f64;
		self.velocity[1] = self.velocity[1] + y as f64;
	}
	pub fn update_acceleration(&mut self, x: i32, y: i32){
		self.acceleration[0] = x as f64;
		self.acceleration[1] = y as f64;
	}
	pub fn set_x(&mut self, x: i32){
		self.pos.x = x;
	}
	pub fn set_y(&mut self, y: i32){
		self.pos.y = y;
	}
	pub fn update_crates(&mut self,game_data: &mut GameData, core :&mut SDLCore, crate_textures: &Vec<Texture>, player: &Player) {
		for c in game_data.crates.iter_mut() {
			// println!("{}, {}", c.velocity[0], c.velocity[1]);
			c.set_x((c.x() as i32 + c.velocity[0] as i32).clamp(game_data.rooms[0].xbounds.0, game_data.rooms[0].xbounds.1));
			c.set_y((c.y() as i32 + c.velocity[1] as i32).clamp(game_data.rooms[0].ybounds.0, game_data.rooms[0].ybounds.1));
			// core.wincan.copy(&crate_textures[0],c.src(),c.offset_pos(player));
		}
	}
	pub fn offset_pos(&self, player:&Player)-> Rect{
		return Rect::new(self.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
		self.y() as i32 + (CENTER_H - player.y() as i32),
		TILE_SIZE, TILE_SIZE);
	}
	// restricts movement of crate when not in contact
	pub fn friction(&mut self){
		if self.x_vel() > 0.0 {
			self.update_velocity(-1.0, 0.0);
		} else if self.x_vel() < 0.0 {
			self.update_velocity(1.0, 0.0);
		}
		if self.y_vel() > 0.0 {
			self.update_velocity(0.0, -1.0);
		} else if self.y_vel() < 0.0 {
			self.update_velocity(0.0, 1.0);
		}
	}
	// calculate velocity resistance
	fn resist(vel: i32, delta: i32) -> i32 {
		if delta == 0 {
			if vel > 0 {-1}
			else if vel < 0 {1}
			else {delta}
		} else {delta}
	}
}