extern crate rogue_sdl;
use crate::gamedata::*;
use crate::{gold,main};
use crate::Player;
use sdl2::rect::Rect;
use rogue_sdl::{Game, SDLCore};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use crate::player::*;
use sdl2::pixels::Color;

pub struct UI<'a>{
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
}

impl<'a> UI<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> UI<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
		UI{
			pos,
			src,
			texture,
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

	pub fn texture(&self) -> &Texture {
        &self.texture
    }


	//update background
	pub fn update_ui(&mut self, player: &Player, core :&mut SDLCore) -> Result<(), String> {
		// set ui bar
		let texture_creator = core.wincan.texture_creator();
		let src = Rect::new(0, 0, CAM_W, TILE_SIZE*2);
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 16, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide_yellow.png")?;
		core.wincan.copy(&ui, src, pos)?;
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 8, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide.png")?;
		core.wincan.copy(&ui, src, pos)?;
		let ttf_creator = sdl2::ttf::init().map_err( |e| e.to_string() )?;
		let get_font = ttf_creator.load_font("font/comic_sans.ttf", 80)?;

		//create hearts
		let mut i=0;
		while i+10 < player.get_hp() {
			let heart = UI::new(
				Rect::new(
					(i/10) as i32 *(TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				), 
				texture_creator.load_texture("images/ui/heart.png")?,
			);
			core.wincan.copy(heart.texture(), heart.src(), heart.pos())?;
			i+=10;
		}
		
		let mut texture = texture_creator.load_texture("images/ui/heart.png")? ;
		if  player.get_hp()%10 != 0  {
			texture = texture_creator.load_texture("images/ui/half_heart.png")?;
		}
			let half_heart = UI::new(
				Rect::new(
					(i/10) as i32 * (TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				),
				texture,
			);
			core.wincan.copy(half_heart.texture(), half_heart.src(), half_heart.pos())?;

		//display mana
		let mut mana = UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE*4)) as i32,
				(CAM_H-(TILE_SIZE)) as i32,
				(TILE_SIZE as f64 / 1.2) as u32,
				(TILE_SIZE as f64 / 1.2) as u32,
			),
			texture_creator.load_texture("images/ui/mana.png")?,
		);
		let cur_mana;
		match player.get_mana() {
			0 => cur_mana = 32 * 4,
			1 => cur_mana = 32 * 3,
			2 => cur_mana = 32 * 2,
			3 => cur_mana = 32 * 1,
			4 => cur_mana = 32 * 0,
			_ => cur_mana = 32 * 0,
		}
		let mana_src = Rect::new(cur_mana, 0, TILE_SIZE / 2, TILE_SIZE / 2);
		mana.set_src(mana_src);
		core.wincan.copy(mana.texture(), mana.src(), mana.pos())?;

		//get current mana as a string
		let mana = player.get_mana();
		let max_mana = player.get_max_mana();
		let mut s: String = mana.to_string();
		let a: String = max_mana.to_string();
		s += "/";
		s += &a;

		//display string next to mana

		//display equipped waepon
		match player.weapon{
			Weapon::Sword=>{ 
				let weapon = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*8) as i32,
						(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
						(TILE_SIZE as f64 * 1.2) as u32,
						(TILE_SIZE as f64 * 1.2) as u32,
					),
					texture_creator.load_texture("images/player/sword_l.png")?,
				);
				core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
			}
			
		}
	match player.ability{
		Ability::Bullet=>{
		let ui_ability = UI::new(
				Rect::new(
					(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*6) as i32,
					(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
					(TILE_SIZE as f64 * 1.2) as u32,
					(TILE_SIZE as f64 * 1.2) as u32,
				),
				texture_creator.load_texture("images/abilities/bullet.png")?,
			);
			core.wincan.copy(ui_ability.texture(), ui_ability.src(),ui_ability.pos())?;
		}
	}
	
		// create coins
		let coin = UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(TILE_SIZE as f64 *1.2) as u32,
				(TILE_SIZE as f64 *1.2) as u32,
			),
			texture_creator.load_texture("images/ui/gold_coin.png")?,
		);
		core.wincan.copy(coin.texture(), coin.src(), coin.pos())?;
		let coin_count = get_font.render( format!("{}", player.get_coins() ).as_str() ).blended(Color::WHITE).unwrap();
		let display_coin_count = texture_creator.create_texture_from_surface( &coin_count ).unwrap();
		core.wincan.copy(&display_coin_count, None, Rect::new( coin.pos().x - 16 as i32, coin.pos().y + 12 as i32, 32, 48) );
																//(text to display, src(none), (positionx, positiony, sizex, sizey))
		Ok(())
	}
}