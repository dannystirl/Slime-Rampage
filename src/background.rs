extern crate rogue_sdl;

use sdl2::render::Texture;
use rand::Rng;

pub struct Background<'a> {
	pub black: Texture<'a>,
	pub texture_0: Texture<'a>,
	pub texture_1: Texture<'a>,
	pub texture_2: Texture<'a>,
	pub texture_3: Texture<'a>,
	pub x_tiles: (i32,i32),
	pub y_tiles: (i32,i32),
	pub tiles: Vec<(bool,i32)>,
}

impl<'a> Background<'a> {
	pub fn new(black: Texture<'a>, texture_0: Texture<'a>, texture_1: Texture<'a>, texture_2: Texture<'a>, texture_3: Texture<'a>, x_tiles: (i32,i32), y_tiles: (i32,i32)) -> Background<'a> {
		let tiles: Vec<(bool,i32)> = vec![(true,0); ((x_tiles.1+2)*(y_tiles.1+1)) as usize]; // (draw?, texture)
		Background {
			black,
			texture_0, 
			texture_1, 
			texture_2, 
			texture_3, 
			x_tiles,
			y_tiles,
			tiles,
		}
	}

	pub fn create_new_map(&mut self, xwalls: (i32,i32), ywalls: (i32,i32)) -> Vec<(i32,i32)> {
		let mut obs: Vec<(i32,i32)> = vec![(0,0);0];
		let mut n = 0;
		for i in 0..xwalls.1+1 {
			for j in 0..ywalls.1+1 {
				if i==0 || i==xwalls.1 || j==0 || j==ywalls.1 { // border
					self.tiles[n].0 = true;
					self.tiles[n].1 = 6;
				} else if i==xwalls.0 || i==xwalls.1-1 || j==ywalls.0 || j==ywalls.1-1 { // border-1 random tiles
					let num = rand::thread_rng().gen_range(0..5);
					self.tiles[n].0 = true;
					self.tiles[n].1 = num;
				} else { // obstacles / nothing
					let num = rand::thread_rng().gen_range(0..75);
					if num==7 && self.tiles[n].0==true { 
						obs.push((i,j));
						self.tiles[n].1 = num;
						// prevent overlap
						self.tiles[n].0 = true;
						self.tiles[n+1].0=false;
						self.tiles[n+ywalls.1 as usize].0=false;
						self.tiles[n+ywalls.1 as usize+1].0=false;
						self.tiles[n+ywalls.1 as usize+2].0=false;

					} else {
						self.tiles[n].0 = false;
					}
				}
				n+=1;
			}
		}
		return obs;
		
	}

	pub fn texture(&self) -> &Texture {
        &self.texture_0
    }
}