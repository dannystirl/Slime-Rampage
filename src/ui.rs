extern crate rogue_sdl;
use crate::gamedata::*;
//use crate::{gold,main};
use crate::Player;
use sdl2::rect::Rect;
use crate::SDLCore;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};
use crate::weapon::*;
use crate::power::*;
use crate::map::*;
use sdl2::pixels::Color;
use sdl2::ttf::Font;

pub struct UI<'a>{
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
}

impl<'a> UI<'a> {
	pub fn new(pos: Rect, texture: Texture<'a>) -> UI<'a> {
		let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE_64 * 2, TILE_SIZE_64);
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
	pub fn update_ui(&mut self, player: &Player, core: &mut SDLCore, map_data: &Map, game_data: &GameData, ui_textures: &Vec<Texture>, get_font: &Font) -> Result<(), String> {
		// set ui bar
		let texture_creator = core.wincan.texture_creator();
		let src = Rect::new(0, 0, CAM_W, TILE_SIZE_64*2);
		let pos = Rect::new(0, (CAM_H - TILE_SIZE_64) as i32 - 16, CAM_W, TILE_SIZE_64*3/2);
		core.wincan.copy(&ui_textures[0], src, pos)?;
		let pos = Rect::new(0, (CAM_H - TILE_SIZE_64) as i32 - 8, CAM_W, TILE_SIZE_64*3/2);
		core.wincan.copy(&ui_textures[1], src, pos)?;

		//create hearts
		let mut i=0;
		while i+10 < player.get_hp() {
			let heart = UI::new(
				Rect::new(
					(i/10) as i32 *(TILE_SIZE_64 as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE_64 as f64 *1.2) as u32) as i32,
					(TILE_SIZE_64 as f64 *1.2) as u32,
					(TILE_SIZE_64 as f64 *1.2) as u32,
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
				(i/10) as i32 * (TILE_SIZE_64 as f64 *1.2) as i32,
				(CAM_H-(TILE_SIZE_64 as f64 *1.2) as u32) as i32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
			),
			texture,
		);
		core.wincan.copy(half_heart.texture(), half_heart.src(), half_heart.pos())?;

		//display mana
		let mut mana = UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE_64*5)) as i32,
				(CAM_H-(TILE_SIZE_64)) as i32,
				(TILE_SIZE_64 as f64 / 1.2) as u32,
				(TILE_SIZE_64 as f64 / 1.2) as u32,
			),
			texture_creator.load_texture("images/ui/mana.png")?,
		);
		let cur_mana;
		match player.get_mana() {
			0 => cur_mana = 32 * 0,
			1 => cur_mana = 32 * 1,
			2 => cur_mana = 32 * 2,
			3 => cur_mana = 32 * 3,
			4 => cur_mana = 32 * 4,
			5..=10 => cur_mana = 32 * 4, 
			_ => cur_mana = 32 * 0,		 // in case there's less than 0 mana somehow. 
		}
		let mana_src = Rect::new(cur_mana, 0, TILE_SIZE_HALF, TILE_SIZE_HALF);
		mana.set_src(mana_src);
		core.wincan.copy(mana.texture(), mana.src(), mana.pos())?;

		let mpos = Rect::new(map_data.ending_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - TILE_SIZE) as i32 / 2, 
							 map_data.ending_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - TILE_SIZE) as i32 / 2, 
							 TILE_SIZE, TILE_SIZE);
		let ppos = Rect::new(player.x() as i32, player.y() as i32, TILE_SIZE_CAM, TILE_SIZE_CAM);
		if GameData::check_collision(&ppos, &mpos) {
			let absorb_help = get_font.render("[E]: Descend Stairs").blended(Color::WHITE).unwrap();
			let display_absorb_help = texture_creator.create_texture_from_surface( &absorb_help ).unwrap();
			core.wincan.copy(&display_absorb_help, None, Rect::new(300 as i32, 660 as i32, 300, 48))?;
		} else {
			// Display helper text for absorption
			if player.can_pickup() {
				let absorb_help = get_font.render("[E]: Absorb Power").blended(Color::WHITE).unwrap();
				let display_absorb_help = texture_creator.create_texture_from_surface( &absorb_help ).unwrap();
				core.wincan.copy(&display_absorb_help, None, Rect::new(300 as i32, 660 as i32, 300, 48))?;
			} else {
				if player.can_pickup_shop() {
					let price_tag = format!("[E]: Buy Item (${})", player.get_shop_price());
					let buy_help = get_font.render(&price_tag).blended(Color::WHITE).unwrap();
					let display_buy_help = texture_creator.create_texture_from_surface( &buy_help ).unwrap();
					core.wincan.copy(&display_buy_help, None, Rect::new(300 as i32, 660 as i32, 300, 48))?;
				} else {
					if player.can_pickup_weapon() {
						let swap_help = get_font.render("[E]: Swap Weapon").blended(Color::WHITE).unwrap();
						let display_swap_help = texture_creator.create_texture_from_surface( &swap_help ).unwrap();
						core.wincan.copy(&display_swap_help, None, Rect::new(300 as i32, 660 as i32, 300, 48))?;
					}
				}
			}
		}

		//display equipped weapon
		match player.get_weapon().weapon_type {
			WeaponType::Sword => { 
				let weapon = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*8) as i32,
						(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32) as i32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
					),
					texture_creator.load_texture("images/weapons/sword.png")?,
				);
				core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
			},
			WeaponType::Spear => {
				let weapon = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*8) as i32,
						(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32) as i32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
					),
					texture_creator.load_texture("images/weapons/spear.png")?,
				);
				core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
			},
			WeaponType::Dagger => {
				let weapon = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*8) as i32,
						(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32) as i32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
						(TILE_SIZE_64 as f64 * 1.2) as u32,
					),
					texture_creator.load_texture("images/weapons/dagger.png")?,
				);
				core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
			}, 
			_ => {}
		}
		
		// Display current power
		match player.get_power().power_type {
			PowerType::Fireball => {
				let ui_ability = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*6) as i32,
						(CAM_H-(TILE_SIZE_64 as u32)) as i32,
						TILE_SIZE_64 as u32,
						TILE_SIZE_64 as u32,
					),
					texture_creator.load_texture("images/abilities/fireball_pickup.png")?,
				);
				core.wincan.copy(ui_ability.texture(), ui_ability.src(),ui_ability.pos())?;
			},
			PowerType::Rock => {
				let ui_ability = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*6) as i32,
						(CAM_H-(TILE_SIZE_64 as u32)) as i32,
						TILE_SIZE_64 as u32,
						TILE_SIZE_64 as u32,
					),
					texture_creator.load_texture("images/abilities/rock.png")?,
				);
				core.wincan.copy(ui_ability.texture(), ui_ability.src(),ui_ability.pos())?;
			},
			PowerType::Slimeball => {
				let ui_ability = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*6) as i32,
						(CAM_H-(TILE_SIZE_64 as u32)) as i32,
						TILE_SIZE_64 as u32,
						TILE_SIZE_64 as u32,
					),
					texture_creator.load_texture("images/abilities/bullet_pickup.png")?,
				);
				core.wincan.copy(ui_ability.texture(), ui_ability.src(), ui_ability.pos())?;
			},
			PowerType::Shield => {
				let ui_ability = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*6) as i32,
						(CAM_H-(TILE_SIZE_64 as u32)) as i32,
						TILE_SIZE_64 as u32,
						TILE_SIZE_64 as u32,
					),
					texture_creator.load_texture("images/abilities/shield_pickup.png")?,
				);
				core.wincan.copy(ui_ability.texture(), ui_ability.src(), ui_ability.pos())?;
			},
			PowerType::Dash => {
				let ui_ability = UI::new(
					Rect::new(
						(CAM_W-((TILE_SIZE_64 as f64 * 1.2) as u32)*6) as i32,
						(CAM_H-(TILE_SIZE_64 as u32)) as i32,
						TILE_SIZE_64 as u32,
						TILE_SIZE_64 as u32,
					),
					texture_creator.load_texture("images/abilities/dash_pickup.png")?,
				);
				core.wincan.copy(ui_ability.texture(), ui_ability.src(), ui_ability.pos())?;
			}
			_ => {},
		}

		let blue_coin = UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE_64 as f64 * 3.0) as u32 - (TILE_SIZE_64 / 3) as u32) as i32,
				(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32 + (TILE_SIZE_64 / 5) as u32) as i32,
				(TILE_SIZE_64 as f64 * 0.8) as u32,
				(TILE_SIZE_64 as f64 * 0.8) as u32,
			),
			texture_creator.load_texture("images/player/slime_old.png")?,
		);
		core.wincan.copy(blue_coin.texture(), blue_coin.src(), blue_coin.pos())?;
		let blue_coin_count = get_font.render(format!("{}", game_data.blue_gold_count).as_str()).blended(Color::WHITE).unwrap();
		let display_blue_coin_count = texture_creator.create_texture_from_surface(&blue_coin_count).unwrap();
		core.wincan.copy(&display_blue_coin_count, None, Rect::new(blue_coin.pos().x - 16 as i32, blue_coin.pos().y + 12 as i32, 32, 48))?;
	
		// create coins
		let coin = UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE_64 as f64 * 1.2) as u32) as i32,
				(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32) as i32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
				(TILE_SIZE_64 as f64 *1.2) as u32,
			),
			texture_creator.load_texture("images/ui/gold_coin.png")?,
		);
		core.wincan.copy(coin.texture(), coin.src(), coin.pos())?;
		let coin_count = get_font.render( format!("{}", player.get_coins() ).as_str() ).blended(Color::WHITE).unwrap();
		let display_coin_count = texture_creator.create_texture_from_surface( &coin_count ).unwrap();
		core.wincan.copy(&display_coin_count, None, Rect::new( coin.pos().x - 16 as i32, coin.pos().y + 12 as i32, 32, 48) )?;

		let mut level_str = format!("Level {}", game_data.current_floor);
		if game_data.current_floor > 3 {
			level_str = "Boss Fight".to_string();
		}
		let level_counter = get_font.render(&level_str).blended(Color::BLUE).unwrap();
		let display_level_counter = texture_creator.create_texture_from_surface( &level_counter ).unwrap();
		core.wincan.copy(&display_level_counter, None, Rect::new(10 as i32, 10 as i32, 150, 48))?;

		if player.god_mode {
			let offset = 10;
			let god_mode = UI::new(
				Rect::new(
					(CAM_W - TILE_SIZE) as i32 - offset,
					0 + offset,
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/ui/god_mode_icon.png")?,
			);
			core.wincan.copy(god_mode.texture(), god_mode.src(), god_mode.pos())?;
		}

		Ok(())
	}
}