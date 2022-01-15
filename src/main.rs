extern crate rogue_sdl;
use rogue_sdl::{Game, SDLCore};
use vector::Vector2D;
use std::time::Duration;
use std::time::Instant;
use std::collections::HashSet;
use std::fs;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};
use sdl2::pixels::Color;
use rand::Rng;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use std::path::Path;
mod background;
mod credits;
mod enemy;
mod gamedata;
mod gold;
mod power;
mod weapon;
mod player;
mod projectile;
mod map;
mod ui;
mod crateobj;
mod rigidbody;
mod vector;

use crate::gamedata::*;
use crate::background::*;
use crate::player::*;
use crate::enemy::*;
use crate::projectile::*;
use crate::power::*;
use crate::weapon::*;
use crate::map::*;
use crate::crateobj::*;
use crate::gold::*;
use crate::ui::*;

pub enum MenuState {
	Title,
	ClassSelection,
	Credits, 
	Store, 
	Play, 
}

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
	class: PlayerType, 
	modifier_type: ModifierType, 
}

// CREATE GAME
impl Game for ROGUELIKE  {

	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		let game_data = GameData::new();
		let class = PlayerType::Jelly; 
		let modifier_type = ModifierType::None; 
		Ok(ROGUELIKE{ core, game_data, class, modifier_type })
	}

	fn run(&mut self) -> Result<(), String> {
		// CREATE GAME CONSTANTS
        let texture_creator = self.core.wincan.texture_creator();

		let title_screen = texture_creator.load_texture("images/menu/title.png")?;
		let class_selection_screen = texture_creator.load_texture("images/menu/class_selection.png")?;
		let shop_screen = texture_creator.load_texture("images/menu/hat_shop.png")?;
		let player_shop = texture_creator.load_texture("images/player/blue_slime_l.png")?;
		let lock = texture_creator.load_texture("images/ui/lock.png")?;

		let mut menu_state = MenuState::Title;
		let mut click_timer = Instant::now();
		let mut credit_timer = Instant::now(); 
		let mut credits_done = false; 

		let mut read_hats = fs::read_to_string("hats.txt").expect("Unable to read file");
    	let mut cowboy = &read_hats[..1];
		let mut gnome = &read_hats[1..2];
		let mut propeller = &read_hats[2..3];

		let ttf_creator = sdl2::ttf::init().map_err( |e| e.to_string() )?;
		let font = ttf_creator.load_font("font/comic_sans.ttf", 80)?;

		'menuloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
						let store = self.game_data.blue_gold_count.to_string();
						fs::write("currency.txt", store).expect("Unable to write file");
						break 'menuloop;
					},
					_ => {},
				}
			}

			let mousestate= self.core.event_pump.mouse_state();
			if mousestate.left() {
				if click_timer.elapsed().as_millis() > 200 {
					click_timer = Instant::now();
					match menu_state {
						MenuState::Title => {
							// SELECT CLASS -> PLAY
							if mousestate.x() >= 107 && mousestate.x() <= 557 &&
								mousestate.y() >= 340 && mousestate.y() <= 424 {
								menu_state = MenuState::ClassSelection;
							// STORE
							} else if mousestate.x() >= 724 && mousestate.x() <= 1174 &&
								mousestate.y() >= 340 && mousestate.y() <= 424 {
									menu_state = MenuState::Store;
							// CREDITS
							} else if mousestate.x() >= 107 && mousestate.x() <= 557 &&
								mousestate.y() >= 458 && mousestate.y() <= 542 {
									credits_done = false; 
									credit_timer = Instant::now(); 
									menu_state = MenuState::Credits;  
							} else if mousestate.x() >= 724 && mousestate.x() <= 1174 &&
								mousestate.y() >= 458 && mousestate.y() <= 542 {
								break 'menuloop;
							}
						},
						MenuState::ClassSelection => {
							if mousestate.x() >= 42 && mousestate.x() <= 415 &&
								mousestate.y() >= 93 && mousestate.y() <= 628 {
								self.class = PlayerType::Jelly;
								menu_state = MenuState::Play; 
							} else if mousestate.x() >= 454 && mousestate.x() <= 827 &&
								mousestate.y() >= 93 && mousestate.y() <= 628 {
								self.class = PlayerType::Warrior;
								menu_state = MenuState::Play; 
							} else if mousestate.x() >= 866 && mousestate.x() <= 1239 &&
								mousestate.y() >= 93 && mousestate.y() <= 628 {
								self.class = PlayerType::Assassin;
								menu_state = MenuState::Play; 
							}
						},
						MenuState::Store => {
							if mousestate.x() >= 900 && mousestate.x() <= 1200 &&
								mousestate.y() >= 500 && mousestate.y() <= 628 {
									if propeller == "t" {
										if self.modifier_type != ModifierType::Fast {
											self.modifier_type = ModifierType::Fast;
										}
										else { self.modifier_type = ModifierType::None; }
									} else {
										if self.game_data.blue_gold_count >= 10 {
											self.game_data.blue_gold_count -= 10;
											let others = &read_hats[..2];
											let unlock = others.to_owned() + "t";
											fs::write("hats.txt", unlock).expect("Unable to write file");
											read_hats = fs::read_to_string("hats.txt").expect("Unable to read file");
    										cowboy = &read_hats[..1];
											gnome = &read_hats[1..2];
											propeller = &read_hats[2..3];
										}
									}
							} else if mousestate.x() >= 100 && mousestate.x() <= 400 &&
								mousestate.y() >= 500 && mousestate.y() <= 628 {
									if cowboy == "t" {
										if self.modifier_type != ModifierType::Heavy {
											self.modifier_type = ModifierType::Heavy;
										}
										else { self.modifier_type = ModifierType::None; }
									} else {
										if self.game_data.blue_gold_count >= 10 {
											self.game_data.blue_gold_count -= 10;
											let others = &read_hats[1..3];
											let unlock = "t".to_owned() + others;
											fs::write("hats.txt", unlock).expect("Unable to write file");
											read_hats = fs::read_to_string("hats.txt").expect("Unable to read file");
    										cowboy = &read_hats[..1];
											gnome = &read_hats[1..2];
											propeller = &read_hats[2..3];
										}
									}
							} else if mousestate.x() >= 500 && mousestate.x() <= 800 &&
								mousestate.y() >= 500 && mousestate.y() <= 628 {
									if gnome == "t" {
										if self.modifier_type != ModifierType::Healthy {
											self.modifier_type = ModifierType::Healthy;
										}
										else { self.modifier_type = ModifierType::None; }
									} else {
										if self.game_data.blue_gold_count >= 10 {
											self.game_data.blue_gold_count -= 10;
											let others_first = &read_hats[..1];
											let others_last = &read_hats[2..3];
											let unlock = others_first.to_owned() + "t" + others_last;
											fs::write("hats.txt", unlock).expect("Unable to write file");
											read_hats = fs::read_to_string("hats.txt").expect("Unable to read file");
    										cowboy = &read_hats[..1];
											gnome = &read_hats[1..2];
											propeller = &read_hats[2..3];
										}
									}
							} else if mousestate.x() >= 20 && mousestate.x() <= 120 &&
								mousestate.y() >= 660 && mousestate.y() <= 708 {
									menu_state = MenuState::Title;
								}
						}
						MenuState::Credits => {
							menu_state = MenuState::Title; 
						}
						MenuState::Play => {
							
						}
					}
				}
			}
			// dislay menu state stuff - ignoring clicks
			match menu_state {
				MenuState::Title => {
					self.core.wincan.copy(&title_screen, None, None)?;
				},
				MenuState::ClassSelection => {
					self.core.wincan.copy(&class_selection_screen, None, None)?;
				}
				MenuState::Store => {
					self.core.wincan.copy(&shop_screen, None, None)?;
					match self.modifier_type {
						ModifierType::Fast => {
							let pos = Rect::new(1000, 100, TILE_SIZE_64, TILE_SIZE_64); 
							let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64); 
							self.core.wincan.copy(&player_shop, src, pos)?;
						}, 
						ModifierType::Healthy => {
							let pos = Rect::new(600, 100, TILE_SIZE_64, TILE_SIZE_64); 
							let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64); 
							self.core.wincan.copy(&player_shop, src, pos)?;
						}, 
						ModifierType::Heavy => {
							let pos = Rect::new(200, 100, TILE_SIZE_64, TILE_SIZE_64); 
							let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64); 
							self.core.wincan.copy(&player_shop, src, pos)?;
						}, 
						_ => {
							
						}
					}

					if cowboy != "t" {
						let cowboy_pos = Rect::new(340, 170, TILE_SIZE_64, TILE_SIZE_64);
						let cowboy_src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
						self.core.wincan.copy(&lock, cowboy_src, cowboy_pos)?;

						let cowboy_price = UI::new(
							Rect::new(
								280,
								175,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
							),
							texture_creator.load_texture("images/player/slime_old.png")?,
						);
						self.core.wincan.copy(cowboy_price.texture(), cowboy_price.src(), cowboy_price.pos())?;
						let cowboy_price_text = font.render(format!("{}", 10).as_str()).blended(Color::WHITE).unwrap();
						let display_cowboy_price = texture_creator.create_texture_from_surface(&cowboy_price_text).unwrap();
						self.core.wincan.copy(&display_cowboy_price, None, Rect::new(cowboy_price.pos().x - 16 as i32, cowboy_price.pos().y + 12 as i32, 32, 48))?;
					}

					if gnome != "t" {
						let gnome_pos = Rect::new(720, 170, TILE_SIZE_64, TILE_SIZE_64);
						let gnome_src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
						self.core.wincan.copy(&lock, gnome_src, gnome_pos)?;

						let gnome_price = UI::new(
							Rect::new(
								660,
								175,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
							),
							texture_creator.load_texture("images/player/slime_old.png")?,
						);
						self.core.wincan.copy(gnome_price.texture(), gnome_price.src(), gnome_price.pos())?;
						let gnome_price_text = font.render(format!("{}", 10).as_str()).blended(Color::WHITE).unwrap();
						let display_gnome_price = texture_creator.create_texture_from_surface(&gnome_price_text).unwrap();
						self.core.wincan.copy(&display_gnome_price, None, Rect::new(gnome_price.pos().x - 16 as i32, gnome_price.pos().y + 12 as i32, 32, 48))?;
					}

					if propeller != "t" {
						let propeller_pos = Rect::new(1120, 170, TILE_SIZE_64, TILE_SIZE_64);
						let propeller_src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
						self.core.wincan.copy(&lock, propeller_src, propeller_pos)?;

						let propeller_price = UI::new(
							Rect::new(
								1060,
								175,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
								(TILE_SIZE_64 as f64 * 0.8) as u32,
							),
							texture_creator.load_texture("images/player/slime_old.png")?,
						);
						self.core.wincan.copy(propeller_price.texture(), propeller_price.src(), propeller_price.pos())?;
						let propeller_price_text = font.render(format!("{}", 10).as_str()).blended(Color::WHITE).unwrap();
						let display_propeller_price = texture_creator.create_texture_from_surface(&propeller_price_text).unwrap();
						self.core.wincan.copy(&display_propeller_price, None, Rect::new(propeller_price.pos().x - 16 as i32, propeller_price.pos().y + 12 as i32, 32, 48))?;
					}

					let blue_coin = UI::new(
						Rect::new(
							(CAM_W-(TILE_SIZE_64 as f64 * 1.0) as u32 - (TILE_SIZE_64 / 3) as u32) as i32,
							(CAM_H-(TILE_SIZE_64 as f64 * 1.2) as u32 + (TILE_SIZE_64 / 5) as u32) as i32,
							(TILE_SIZE_64 as f64 * 0.8) as u32,
							(TILE_SIZE_64 as f64 * 0.8) as u32,
						),
						texture_creator.load_texture("images/player/slime_old.png")?,
					);
					self.core.wincan.copy(blue_coin.texture(), blue_coin.src(), blue_coin.pos())?;
					let blue_coin_count = font.render(format!("{}", self.game_data.blue_gold_count).as_str()).blended(Color::WHITE).unwrap();
					let display_blue_coin_count = texture_creator.create_texture_from_surface(&blue_coin_count).unwrap();
					self.core.wincan.copy(&display_blue_coin_count, None, Rect::new(blue_coin.pos().x - 16 as i32, blue_coin.pos().y + 12 as i32, 32, 48))?;

					let back_text = "Back";
					let back_button = font.render(&back_text).blended(Color::WHITE).unwrap();
					let display_back_button = texture_creator.create_texture_from_surface( &back_button ).unwrap();
					self.core.wincan.copy(&display_back_button, None, Rect::new(20 as i32, 660 as i32, 100, 48))?;
				}
				MenuState::Credits => {
					for event in self.core.event_pump.poll_iter() {
						match event {
							Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => { menu_state = MenuState::Title; credits_done=true; },
							Event::KeyDown{keycode: Some(Keycode::Q), ..} => { menu_state = MenuState::Title; credits_done=true; },
							_ => {},
						}
					}
					let texture;
					let scene_time = 200; // how long each credit scene lasts. 
					match ((credit_timer.elapsed().as_millis()/scene_time) / 11) as i32  {
						0 => {
						// Title
							texture = texture_creator.load_texture("images/credits/credits_title.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						1 => {
						// Physics Engine Team
							texture = texture_creator.load_texture("images/credits/credits_physics.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						2 => {
						// Davon Allensworth
							texture = texture_creator.load_texture("images/credits/credits_davon.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						3 => {
						// Zirui Huang
							texture = texture_creator.load_texture("images/credits/zih_credit.jpg")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						4 => {
						// Victor Mui
							texture = texture_creator.load_texture("images/credits/credits_victor.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						5 => {
						// Adam Wachowicz
							texture = texture_creator.load_texture("images/credits/credits_adam.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						6 => {
						// Procedural Generation Team
							texture = texture_creator.load_texture("images/credits/credits_procedural_gen.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						7 => {
						// Yihua Pu
							texture = texture_creator.load_texture("images/credits/Yihua_credit.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						8 => {
						// Marshall Lentz
							texture = texture_creator.load_texture("images/credits/credits_marshall.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						9 => {
						// Josh Friedman
							texture = texture_creator.load_texture("images/credits/friedman_credits.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						10 => {
						// Daniel Stirling
							texture = texture_creator.load_texture("images/credits/credits_daniel.png")?;
							self.core.wincan.copy(&texture, None, None)?;
						}
						_ => { credits_done = true; } 
					}
					if credits_done { menu_state = MenuState::Title; }
				}
				MenuState::Play => {
					let texture_creator = self.core.wincan.texture_creator();
					// AUDIO SYSTEM; TAKEN FROM SDL2 MIXER DEMO FOR RUST
					let frequency = 44_100;
					let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
					let channels = DEFAULT_CHANNELS; // Stereo
					let chunk_size = 1_024;
					sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
					let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
					// Number of mixing channels available for sound effect `Chunk`s to play simultaneously.
					sdl2::mixer::allocate_channels(4);

					let path = Path::new("./music/Rampage.wav");
					let music = sdl2::mixer::Music::from_file(path)?;
					music.play(-1)?;
					
					let modifier = Modifier::new(self.modifier_type);
					let mod_texture; 
					match self.modifier_type {
						ModifierType::Fast => {
							mod_texture = texture_creator.load_texture("images/player/propeller_hat.png").unwrap(); 
						}, 
						ModifierType::Healthy => {
							mod_texture = texture_creator.load_texture("images/player/gnome_hat.png").unwrap(); 
						}, 
						ModifierType::Heavy => {
							mod_texture = texture_creator.load_texture("images/player/ten_gallon.png").unwrap(); 
						}, 
						_ => {
							mod_texture = texture_creator.load_texture("images/player/ten_gallon.png").unwrap()
						}
					}

					// create player 
					let mut player: Player; 
					#[allow(unreachable_patterns)]
					match self.class {
						PlayerType::Warrior => {
							player = player::Player::new(
								texture_creator.load_texture("images/player/green_slime_sheet.png").unwrap(), 
								mod_texture, 
								self.class,
								modifier, 
							);
						}, 
						PlayerType::Assassin => {
							player = player::Player::new(
								texture_creator.load_texture("images/player/pink_slime_sheet.png").unwrap(), 
								mod_texture, 
								self.class,
								modifier, 
							);
						}, 
						PlayerType::Jelly => {
							player = player::Player::new(
								texture_creator.load_texture("images/player/blue_slime_sheet.png").unwrap(), 
								mod_texture, 
								self.class,
								modifier, 
							);
						}, 
						_ => {
							player = player::Player::new(
								texture_creator.load_texture("images/player/blue_slime_sheet.png").unwrap(), 
								mod_texture, 
								self.class,
								modifier, 
							);
						}, 
					};
					
					let mut rng = rand::thread_rng();
					// create ui
					let mut ui = ui::UI::new(
						Rect::new(
							(10) as i32 *(TILE_SIZE_64 as f64 *1.2) as i32,
							(CAM_H-(TILE_SIZE_64 as f64 *1.2) as u32) as i32,
							(TILE_SIZE_64 as f64 *1.2) as u32,
							(TILE_SIZE_64 as f64 *1.2) as u32,
						), 
						texture_creator.load_texture("images/ui/heart.png")?,
					);
					// LOAD TEXTURES
					// projectile textures
					let mut ability_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
					let bullet_player = texture_creator.load_texture("images/abilities/bullet_player.png")?; 
					let bullet_enemy = texture_creator.load_texture("images/abilities/bullet_enemy.png")?;
					let fireball = texture_creator.load_texture("images/abilities/new_fireball.png")?;
					let shield = texture_creator.load_texture("images/abilities/shield_outline.png")?;
					let wall = texture_creator.load_texture("images/abilities/wall.png")?;
					let shrapnel = texture_creator.load_texture("images/objects/shrapnel.png")?;
					let rock = texture_creator.load_texture("images/abilities/rock.png")?;
					ability_textures.push(bullet_player);
					ability_textures.push(fireball);
					ability_textures.push(bullet_enemy);
					ability_textures.push(shield);
					ability_textures.push(wall);
					ability_textures.push(shrapnel);
					ability_textures.push(rock);
					// object textures
					let mut crate_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
					let crate_texture = texture_creator.load_texture("images/objects/crate.png")?;
					let heavy = texture_creator.load_texture("images/objects/metal_crate.png")?;
					let explosive = texture_creator.load_texture("images/objects/new_barrel.png")?;
					crate_textures.push(crate_texture);
					crate_textures.push(heavy);
					crate_textures.push(explosive);
					
					let coin_texture = texture_creator.load_texture("images/ui/gold_coin.png")?;
					let gold_coin_texture = texture_creator.load_texture("images/player/slime_old.png")?;
					let fireball_texture = texture_creator.load_texture("images/abilities/fireball_pickup.png")?;
					let slimeball_texture = texture_creator.load_texture("images/abilities/bullet_pickup.png")?;
					let shield_texture = texture_creator.load_texture("images/abilities/shield_pickup.png")?;
					let dash_texture = texture_creator.load_texture("images/abilities/dash_pickup.png")?;
					let sword_texture = texture_creator.load_texture("images/weapons/sword.png")?;
					let spear_texture = texture_creator.load_texture("images/weapons/spear.png")?;
					let dagger_texture = texture_creator.load_texture("images/weapons/dagger.png")?;
					let health_texture = texture_creator.load_texture("images/ui/heart.png")?; 
					let health_upgrade_texture = texture_creator.load_texture("images/ui/heart_upgrade.png")?;
					let mana_upgrade_texture = texture_creator.load_texture("images/ui/mana_upgrade.png")?;
					let rock_texture = texture_creator.load_texture("images/abilities/rock.png")?; //need to change it to a new texture


					let mut ui_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
					let uibb = texture_creator.load_texture("images/ui/bb_wide_yellow.png")?;
					ui_textures.push(uibb);//0
					let uibb = texture_creator.load_texture("images/ui/bb_wide.png")?;
					ui_textures.push(uibb);//1
					let full = texture_creator.load_texture("images/ui/heart.png")? ;
					ui_textures.push(full);//2
					let half = texture_creator.load_texture("images/ui/half_heart.png")?;
					ui_textures.push(half);//3
					ui_textures.push(texture_creator.load_texture("images/ui/mana.png")?	);//4

					// MAIN GAME LOOP
					'gameloop: loop {
						// CREATE MAPS
						let background = background::Background::new(
							texture_creator.load_texture("images/background/bb.png")?,
							texture_creator.load_texture("images/background/floor_tile_1.png")?,
							texture_creator.load_texture("images/background/floor_tile_2.png")?,
							texture_creator.load_texture("images/background/tile.png")?,
							texture_creator.load_texture("images/background/skull.png")?,
							texture_creator.load_texture("images/background/upstairs.png")?,
							texture_creator.load_texture("images/background/downstairs.png")?,
							texture_creator.load_texture("images/background/dirt_sheet.png")?,
							Rect::new(
								(0 + ((TILE_SIZE_CAM / 2) as i32)) - ((CAM_W / 2) as i32),
								(0 + ((TILE_SIZE_CAM / 2) as i32)) - ((CAM_H / 2) as i32),
								CAM_W,
								CAM_H,
							),
						);
						let mut map_data = map::Map::new(self.game_data.current_floor, background);
						if self.game_data.current_floor == 4 {	// boss level
							map_data.create_boss();
						} else if self.game_data.current_floor == 5 {	// post boss level
							let store = self.game_data.blue_gold_count.to_string();
							fs::write("currency.txt", store).expect("Unable to write file");
							self.game_data = GameData::new();
							break 'gameloop; 
						} else {	// regular levels
							map_data.create_map();
						}

						// set starting position
						player.set_x((map_data.starting_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);
						player.set_y((map_data.starting_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);

						// reset arrays
						self.game_data.crates = Vec::<Crate>::with_capacity(0);
						self.game_data.dropped_powers = Vec::<Power>::with_capacity(0);
						self.game_data.gold = Vec::<Gold>::with_capacity(0);
						self.game_data.blue_gold = Vec::<Gold>::with_capacity(0);
						self.game_data.player_projectiles = Vec::<Projectile>::with_capacity(0);
						self.game_data.enemy_projectiles = Vec::<Projectile>::with_capacity(0);
						// OBJECT GENERATION
						if DEVELOP {
							let pos = Rect::new(
								player.x() as i32 -200, 
								player.y() as i32, 
								TILE_SIZE,
								TILE_SIZE
							);
							self.game_data.crates.push(crateobj::Crate::new(pos, CrateType::Explosive));
						}

						// create enemies
						let mut enemies: Vec<Enemy> = Vec::new();
						let mut rngt = Vec::new();

						let mut enemy_count = 0;
						let max_h = MAP_SIZE_H; 
						let max_w = MAP_SIZE_W;
						for h in 0..max_h {
							for w in 0..max_w {
								if map_data.enemy_and_object_spawns[h][w] == 0 {
									continue;
								}
								match map_data.enemy_and_object_spawns[h][w] {
									1 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												TILE_SIZE_CAM,
												TILE_SIZE_CAM
											),
											texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
											EnemyType::Melee,
											enemy_count,
											self.game_data.current_floor, 
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									2 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												TILE_SIZE_CAM,
												TILE_SIZE_CAM
											),
											texture_creator.load_texture("images/enemies/ranged_enemy.png")?,
											EnemyType::Gellem,
											enemy_count,
											self.game_data.current_floor, 
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									3 => {
										let roll= rng.gen_range(0..10);
										let c: crateobj::Crate; 
										match roll {
											0..=2 => {
												c = crateobj::Crate::new(
													Rect::new(
														w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
														h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
														62,
														62
													), 
													CrateType::Heavy, 
												); 
											}
											3..=5 => {
												c = crateobj::Crate::new(
													Rect::new(
														w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
														h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
														TILE_SIZE_PLAYER*3/2,
														TILE_SIZE_PLAYER*3/2
													), 
													CrateType::Explosive, 
												);
											}
											_ => {
												c = crateobj::Crate::new(
													Rect::new(
														w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
														h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
														TILE_SIZE_PLAYER,
														TILE_SIZE_PLAYER
													), 
													CrateType::Regular, 
												);
											}
										}
										self.game_data.crates.push(c);
									}
									4 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												TILE_SIZE_CAM,
												TILE_SIZE_CAM
											),
											texture_creator.load_texture("images/enemies/Shield_skeleton.png")?,
											EnemyType::Skeleton,
											enemy_count,
											self.game_data.current_floor, 
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									5 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												TILE_SIZE_CAM,
												TILE_SIZE_CAM
											),
											texture_creator.load_texture("images/enemies/eyeball.png")?,
											EnemyType::Eyeball,
											enemy_count,
											self.game_data.current_floor, 
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									6 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												TILE_SIZE_CAM,
												TILE_SIZE_CAM
											),
											texture_creator.load_texture("images/enemies/rock.png")?,
											EnemyType::Rock,
											enemy_count,
											self.game_data.current_floor,
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									7 => {
										let e = enemy::Enemy::new(
											Rect::new(
												w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
												h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
												128,
												128
											),
											texture_creator.load_texture("images/enemies/boss.png")?,
											EnemyType::Boss,
											enemy_count,
											self.game_data.current_floor, 
										);
										enemies.push(e);
										rngt.push(rng.gen_range(1..5));
										enemy_count += 1;
									}
									_ => {}
								}
							}
						}

						if DEBUG {
							let mut total_health = 0; 
							for enemy in enemies.iter_mut() {
								total_health += enemy.hp; 
							}
							if enemy_count != 0 {
								println!("average enemy health of {} enemies on floor {}: {}", enemy_count as i32, self.game_data.current_floor, total_health/enemy_count as i32)
							}
							// average floor 1 health: 15
							// enemies gain about 12 health per level
						}

						let mut all_frames = 0;
						let last_time = Instant::now();

						// INDIVIDUAL LEVEL LOOP
						'level: loop {
							for event in self.core.event_pump.poll_iter() {
								match event {
									Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
										let store = self.game_data.blue_gold_count.to_string();
										fs::write("currency.txt", store).expect("Unable to write file");
										break 'gameloop; 
									}, 
									_ => {},
								}
							}
							// fps calculations
							let mut fps_avg: f64 = 60.0; 
							all_frames += 1;
							let elapsed = last_time.elapsed();
							if elapsed > Duration::from_secs(1) {
								fps_avg = (all_frames as f64) / elapsed.as_secs_f64();
								self.game_data.set_speed_limit(fps_avg.recip() * SPEED_LIMIT);
								self.game_data.set_accel_rate(fps_avg.recip() * ACCEL_RATE);
							}
							// reset frame values
							player.set_x_accel(0);
							player.set_y_accel(0);

							// Array that adds newly spawned shrapnel to projectiles
							// I had to do this because i cant add to a list while iterating over it (thanks Rust)
							let mut explosion_shrapnel = Vec::<Projectile>::with_capacity(0);

							// Draw background
							self.core.wincan.copy(&map_data.background.black, None, None)?;

							// GET INPUT
							let mousestate= self.core.event_pump.mouse_state();
							let keystate: HashSet<Keycode> = self.core.event_pump
								.keyboard_state()
								.pressed_scancodes()
								.filter_map(Keycode::from_scancode)
								.collect();
							if keystate.contains(&Keycode::E){
								let mpos = Rect::new(map_data.ending_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - TILE_SIZE) as i32 / 2, 
								map_data.ending_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - TILE_SIZE) as i32 / 2, 
								TILE_SIZE, TILE_SIZE);
								let ppos = Rect::new(player.x() as i32, player.y() as i32, TILE_SIZE_CAM, TILE_SIZE_CAM);
								if check_collision(&ppos, &mpos) {
									break 'level
								}
							}
							ROGUELIKE::check_inputs(self, &keystate, mousestate, &mut player, fps_avg, &mut map_data)?;

							// UPDATE BACKGROUND
							ROGUELIKE::draw_background(self, &player, &mut map_data.background, map_data.map, map_data.colored_map)?;

							// UPDATE PLAYER
							player.update_player(&self.game_data, map_data.map, &mut self.core)?;
							ROGUELIKE::draw_player(self, fps_avg, &mut player, map_data.background.get_curr_background());
							ROGUELIKE::update_crates(self, &crate_textures, &mut player, map_data.map);

							// UPDATE ENEMIES
							rngt = ROGUELIKE::update_enemies(self, &mut rngt, &mut enemies, &player, map_data.map);
							// UPDATE ATTACKS
							ROGUELIKE::update_projectiles(&mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
							ROGUELIKE::draw_enemy_projectile(self, &ability_textures, &player);	
							ROGUELIKE::draw_player_projectile(self, &ability_textures,  &player, mousestate)?;	
							ROGUELIKE::draw_weapon(self, &player, &sword_texture, &spear_texture, &dagger_texture);
							
							// UPDATE INTERACTABLES
							ROGUELIKE::update_drops(self, &mut enemies, &mut player, &mut map_data, &coin_texture, &gold_coin_texture, 
													&fireball_texture, &slimeball_texture, &shield_texture,
													&dash_texture, &health_texture, &health_upgrade_texture,
													&sword_texture, &spear_texture, &dagger_texture, &rock_texture,
													&mana_upgrade_texture);

							// CHECK COLLISIONS
							ROGUELIKE::check_collisions(self, &mut player, &mut enemies, &mut map_data, &crate_textures, fps_avg, &mut explosion_shrapnel);
							if player.is_dead(){
								self.game_data = GameData::new();
								break 'gameloop;
							}

							// Check if any shrapnel has been added and append
							if explosion_shrapnel.len() > 0{
								for scrap in explosion_shrapnel{
									self.game_data.player_projectiles.push(scrap);
								}
							}

							// UPDATE UI
							ui.update_ui(&player, &mut self.core, &map_data, &self.game_data, &ui_textures, &font)?;
							
							// UPDATE FRAME
							self.core.wincan.present();
						}
						// give player permanent coins upon beating a level
						let store = (self.game_data.blue_gold_count+(1*self.game_data.current_floor as u32)).to_string();
						fs::write("currency.txt", store).expect("Unable to write file");
						self.game_data.current_floor += 1;
						self.game_data.map_size_w = 61 + ((self.game_data.current_floor-1)*30) as usize;
						self.game_data.map_size_h = 61 + ((self.game_data.current_floor-1)*30) as usize;
					}
					menu_state = MenuState::Title; 
					self.game_data = GameData::new();
				}
			}
			self.core.wincan.present();
		}
		Ok(())
	}
}

pub fn main() -> Result<(), String> {
	rogue_sdl::runner(TITLE, ROGUELIKE::init);
	Ok(())
}

// check collision
fn check_collision(a: &Rect, b: &Rect) -> bool {
	if a.bottom() < b.top()
		|| a.top() > b.bottom()
		|| a.right() < b.left()
		|| a.left() > b.right()
	{
		false
	}
	else {
		true
	}
}

// Create map
impl ROGUELIKE {
	pub fn draw_background(&mut self, player: &Player, background: &mut Background, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], dirt_map: [[(i32,i32); MAP_SIZE_W]; MAP_SIZE_H]) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let _floor = texture_creator.load_texture("images/background/floor_tile_1.png")?;
		let dirt_sheet = texture_creator.load_texture("images/background/dirt_sheet.png")?;
		let shop = texture_creator.load_texture("images/background/floor_tile_maroon.png")?;
		let tile = texture_creator.load_texture("images/background/tile.png")?;
		let moss_tile = texture_creator.load_texture("images/background/moss_tile.png")?;
		let upstairs = texture_creator.load_texture("images/background/new_stairs.png")?;
		let downstairs = texture_creator.load_texture("images/background/new_stairs.png")?;
		background.set_curr_background(player.x(), player.y(), player.width(), player.height());

		let h_bounds_offset = (player.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (player.x() / TILE_SIZE as f64) as i32;
	
		for h in 0..(CAM_H / TILE_SIZE) + 1 {
			for w in 0..(CAM_W / TILE_SIZE) + 1 {
				let mut src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
				let pos = Rect::new(w as i32 * TILE_SIZE as i32 - (player.x() % TILE_SIZE as f64) as i32,
									h as i32 * TILE_SIZE as i32 - (player.y() % TILE_SIZE as f64) as i32,
									TILE_SIZE, TILE_SIZE);
				if h as i32 + h_bounds_offset < 0 ||
					w as i32 + w_bounds_offset < 0 ||
					h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
					w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
					map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
					continue;
				} else{
					let num = map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize];
					match num {
						1 => { // floor tiles
							src.x = dirt_map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize].0*72; 
							src.y = dirt_map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize].1*72; 
							self.core.wincan.copy_ex(&dirt_sheet, src, pos, 0.0, None, false, false).unwrap(); 
						},
						2 => { self.core.wincan.copy_ex(&tile, src, pos, 0.0, None, false, false).unwrap(); },  		// tile tiles
						5 => { self.core.wincan.copy_ex(&moss_tile, src, pos, 0.0, None, false, false).unwrap(); },  	// moss tiles
						6 => { self.core.wincan.copy_ex(&shop, src, pos, 0.0, None, false, false).unwrap(); },  		// shop tile
						3 => { self.core.wincan.copy_ex(&upstairs, src, pos, 0.0, None, false, false).unwrap(); },  	// upstairs tile
						4 => { 
							if self.game_data.current_floor == 4 && !self.game_data.boss_killed {
								self.core.wincan.copy_ex(&dirt_sheet, src, pos, 0.0, None, false, false).unwrap(); 		// hide exit stairs
							} else {
								self.core.wincan.copy_ex(&downstairs, src, pos, 0.0, None, false, false).unwrap();   	// downstairs tile
							}
						}
						_ => {  }, 
					}
				}
			}
		}
		Ok(())
	}
	
	// update enemies
	pub fn update_enemies(&mut self, rngt: &mut Vec<i32>, enemies: &mut Vec<Enemy>, player: &Player,map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Vec<i32> {
		let mut i = 0;
		for enemy in enemies {
			if enemy.is_alive(){
				enemy.check_attack(&mut self.game_data, (player.x(), player.y()));
				// direction changer
				if self.game_data.frame_counter.elapsed().as_millis() % 120 as u128 == 0 as u128 || enemy.force_move(map) { 
					rngt[i] = rand::thread_rng().gen_range(1..5);
				}
				enemy.update_enemy(&self.game_data, rngt, i, (player.x(), player.y()), map);
				self.core.wincan.copy_ex(enemy.txtre(), enemy.src(), enemy.offset_pos(player), 0.0, None, enemy.facing_right, false).unwrap();
				i += 1;
			}
		}
		return rngt.to_vec();
	}

	pub fn update_crates(&mut self, crate_textures: &Vec<Texture>, player: &Player, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]){
		for c in self.game_data.crates.iter_mut(){
			if c.is_active() {
				c.update_crates(&mut self.core, crate_textures, player, map);
			}
		}
	}
	
	#[allow(unused_variables)]
	pub fn update_drops(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map_data: &mut Map, coin_texture: &Texture, blue_coin_texture: &Texture, 
						fireball_texture: &Texture, slimeball_texture: &Texture, shield_texture: &Texture,
						dash_texture: &Texture, health_texture: &Texture, health_upgrade_texture: &Texture,
						sword_texture: &Texture, spear_texture: &Texture, dagger_texture: &Texture,  rock_texture: &Texture,
						mana_upgrade_texture: &Texture) {
		//add enemy drops to game
		for enemy in enemies {
			if !enemy.is_alive() && enemy.has_item() {
				if enemy.has_money() {
					let money = enemy.money(); 
					for i in 0..money {
						match enemy.enemy_type {
							EnemyType::Boss => {
								self.game_data.blue_gold.push(enemy.drop_coin());
							}
							_ => {
								self.game_data.gold.push(enemy.drop_coin());
							}
						}
					}
				}
				if enemy.has_power() {
					self.game_data.dropped_powers.push(enemy.drop_power());
				}
			}
		}
		// draw uncollected coins
		for coin in self.game_data.gold.iter_mut() {
			if !coin.collected() {
				let pos = Rect::new(coin.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									coin.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&coin_texture, coin.src(), pos, 0.0, None, false, false).unwrap();
			}
		}
		for coin in self.game_data.blue_gold.iter_mut() {
			if !coin.collected() {
				let pos = Rect::new(coin.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									coin.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&blue_coin_texture, coin.src(), pos, 0.0, None, false, false).unwrap();
			}
		}

		// draw powers
		for power in self.game_data.dropped_powers.iter_mut() {
			if !power.collected() {
				let pos = Rect::new(power.x() as i32 + (CENTER_W - player.x() as i32),
									power.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE_POWER, TILE_SIZE_POWER);
				match power.power_type() {
					PowerType::Fireball => {
						self.core.wincan.copy_ex(&fireball_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					PowerType::Slimeball => {
						self.core.wincan.copy_ex(&slimeball_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					PowerType::Shield => {
						self.core.wincan.copy_ex(&shield_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					PowerType::Dash => {
                    	self.core.wincan.copy_ex(&dash_texture, power.src(), pos, 0.0, None, false, false).unwrap();
                    },
                    PowerType::Rock => {
                        self.core.wincan.copy_ex(&rock_texture, power.src(), pos, 0.0, None, false, false).unwrap();
					},
					_ => {},
				}
			}
		}

		// draw shop items
		let mut i = 0; 
		while i < map_data.shop_spawns.len() {
			if map_data.shop_items[i].1 {
				i += 1;
				continue;
			}
			let src = Rect::new(0,0,TILE_SIZE_64,TILE_SIZE_64); 
			let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - player.x() as i32,
								(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - player.y() as i32,
								TILE_SIZE_POWER, TILE_SIZE_POWER);
			match map_data.shop_items[i].0 {
				ShopItems::Fireball => {
					self.core.wincan.copy_ex(&fireball_texture, src, pos, 0.0, None, false, false).unwrap();
				},
				ShopItems::Slimeball => {
					self.core.wincan.copy_ex(&slimeball_texture, src, pos, 0.0, None, false, false).unwrap();
				},
				ShopItems::Shield => {
					self.core.wincan.copy_ex(&shield_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Dash => {
					self.core.wincan.copy_ex(&dash_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Sword => {
					self.core.wincan.copy_ex(&sword_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Spear => {
					self.core.wincan.copy_ex(&spear_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Dagger => {
					self.core.wincan.copy_ex(&dagger_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::HealthUpgrade => {
					self.core.wincan.copy_ex(&health_upgrade_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Health => {
					self.core.wincan.copy_ex(&health_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::ManaUpgrade => {
					self.core.wincan.copy_ex(&mana_upgrade_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Rock => {
                    self.core.wincan.copy_ex(&rock_texture, src, pos, 0.0, None, false, false).unwrap();
                },
				_ => {}
			}
			i += 1; 
		}
	}

	// check input values
	#[allow(unused_assignments)]
	pub fn check_inputs(&mut self, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player, fps_avg: f64, map_data: &mut Map)-> Result<(), String>  {
		// move up
		if keystate.contains(&Keycode::W) {
			player.rb.accel.y = player.rb.accel.y-self.game_data.get_accel_rate();
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.rb.accel.x = player.rb.accel.x-self.game_data.get_accel_rate();
			player.facing_right = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.rb.accel.y = player.rb.accel.y+self.game_data.get_accel_rate();
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.rb.accel.x = player.rb.accel.x+self.game_data.get_accel_rate();
			player.facing_right = true;
		}
		// basic attack
		if keystate.contains(&Keycode::Space) {
			if !(player.get_attacking()) {
				player.attack();
			}
		}
		// Shoot ranged attack
		if mousestate.left(){
			match player.get_power().power_type {
				PowerType::Fireball => {
					if !player.is_firing && player.get_mana() >= player.get_power().mana_cost {
						let now = Instant::now();
						let elapsed = now.elapsed().as_millis() / (fps_avg as u128 * 2 as u128); // the bigger this divisor is, the faster the animation plays
						let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), PowerType::Fireball, elapsed);
						self.game_data.player_projectiles.push(bullet);
					}
				},
				PowerType::Slimeball => {
					if !player.is_firing && player.get_mana() >= player.get_power().mana_cost {
						let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), PowerType::Slimeball, 0);
						self.game_data.player_projectiles.push(bullet);
					}
				},
				PowerType::Shield => {
					if !player.get_shielded() && player.get_mana() >= player.get_power().mana_cost {
						player.set_shielded(true);
						// IN PROGRESS code for placeable shield
						/* let bullet = player.fire(player.x() as i32, player.y() as i32, 0.0, PowerType::Shield, 0);
						self.game_data.player_projectiles.push(bullet); */
					}
				},
				PowerType::Dash => {
                    if !player.is_firing && player.get_mana() >= player.get_power().mana_cost {
                        player.set_dash_timer();
                    }
                },
                PowerType::Rock => {
                    if !player.is_firing && player.get_mana() >= player.get_power().mana_cost {
                        let rock = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), PowerType::Rock, 0);
                        self.game_data.player_projectiles.push(rock);
                    }
                },
				/* PowerType::Wall => { // placeable shield IN PROGRESS
					if player.get_mana() >= player.get_power().mana_cost {
						let bullet = player.fire(player.x() as i32, player.y() as i32, 0.0, PowerType::Wall, 0);
						self.game_data.player_projectiles.push(bullet); 
					}
				}, */
				_ => {},
			}
		}
		// Absorb power
		if keystate.contains(&Keycode::E) {
			if player.can_pickup() || player.can_pickup_shop() || player.can_pickup_weapon() {
				let mut picked_up = false;
				for drop in self.game_data.dropped_powers.iter_mut() {
					if check_collision(&player.pos(), &drop.pos()) &&
					   !drop.collected() && player.get_pickup_timer() > 1000 {
						drop.set_collected();
						player.reset_pickup_timer();
						player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), *drop.power_type()));
						picked_up = true;
						break;
					}
				}
				if !picked_up {
					let mut i = 0; 
					while i < map_data.shop_spawns.len() {
						if map_data.shop_items[i].1 {
							i += 1;
							continue;
						}
						let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
											(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
											TILE_SIZE, TILE_SIZE);
						if check_collision(&player.pos(), &pos) && player.get_pickup_timer() > 1000 &&
						(player.get_coins() >= map_data.shop_items[i].2 || map_data.shop_items[i].1 == true ) {
							player.reset_pickup_timer();
							player.sub_coins(map_data.shop_items[i].2);
							// [i].0 - item type
							// [i].1 - item has been purchased. used for selectable items 
							match map_data.shop_items[i].0 {
								ShopItems::Fireball => {
									player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Fireball));
								},
								ShopItems::Slimeball => {
									player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Slimeball));
								},
								ShopItems::Shield => {
									player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Shield));
								}
								ShopItems::Dash => {
									player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Dash));
								}
								ShopItems::Rock => {
                                    player.set_power(Power::new(Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE), PowerType::Rock));
                                },
								ShopItems::Sword => {
									if map_data.shop_items[i].1 == false {
										if player.get_weapon().weapon_type != WeaponType::Sword {
											player.set_weapon(WeaponType::Sword)
										} else {
											player.weapon.upgrade_weapon_damage(5);
										}
									}
								}
								ShopItems::Spear => {
									if map_data.shop_items[i].1 == false {
										if player.get_weapon().weapon_type != WeaponType::Spear {
											player.set_weapon(WeaponType::Spear)
										} else {
											player.weapon.upgrade_weapon_damage(8);
										}
									}
								}
								ShopItems::HealthUpgrade => {
									if map_data.shop_items[i].1 == false {
										player.upgrade_hp(10); 
										player.plus_hp(10); 
									} 
								}
								ShopItems::Health => {
									player.plus_hp(10);
								}
								ShopItems::ManaUpgrade => {
									if map_data.shop_items[i].1 == false {
										player.upgrade_mana(); 
										player.restore_mana(); 
									} 
								}
								_ => { }
							}
							map_data.shop_items[i].1 = true;
							picked_up = true;
							break;
						}
						i+=1; 
					}
				}
			}
		}
		// Toggle god mode
		if keystate.contains(&Keycode::G) {
			if player.get_god_mode_timer() > 250 {
				player.god_mode = !player.god_mode;
				player.set_god_mode_timer();
			}
		}
		// FOR TESTING ONLY: USE TO FOR PRINT VALUES
		if keystate.contains(&Keycode::P) {
			println!("{} {}", player.x(), player.y());	
			for item in map_data.shop_spawns.iter() {
				let pos = Rect::new((item.1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
									(item.0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
									TILE_SIZE, TILE_SIZE);
				println!("{},{}", pos.x, pos.y);
			}
		}
		Ok(())	
	}

	// update projectiles
	pub fn update_projectiles(player_projectiles: &mut Vec<Projectile>, enemy_projectiles: &mut Vec<Projectile>) {
		for projectile in player_projectiles {
			if projectile.is_active() {
				projectile.update_pos();
			}
		}
		for projectile in enemy_projectiles {
			if projectile.is_active() {
				projectile.update_pos();
			}
		}
	}
	
	// check collisions
	fn check_collisions(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>, map_data: &mut Map, _crate_textures: &Vec<Texture>, _fps_avg: f64, explosion_shrapnel: &mut Vec<Projectile>) {
		let map = map_data.map;

		// PLAYER COLLISION VS ENEMY COLLISION
		for enemy in enemies.iter_mut() {
			if enemy.is_alive() {
				if check_collision(&player.rb.pos(), &enemy.pos()) {
					player.minus_hp(enemy.collision_damage);
				}
				// player melee collisions
				if player.get_attacking() {
					if check_collision(&player.get_attack_box(), &enemy.pos()) {
						enemy.knockback(player.x().into(), player.y().into());
						if enemy.minus_hp(player.get_weapon().damage) { self.game_data.boss_killed = true; }
					}
				}
			}
		}

		// PLAYER COLLISION VS CRATE COLLISION
		for c in self.game_data.crates.iter_mut(){
			if c.is_active() {
				let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
				let pen = &mut 0.0;
				if player.rb.rect_vs_rect(c.rb, normal_collision, pen) {
					// provide impulse
					player.rb.resolve_col(&mut c.rb, *normal_collision, *pen);
				} else {
					c.friction();
				}
			}
		}

		// ENEMIES COLLISION VS CRATE COLLISION
		for c in self.game_data.crates.iter_mut() {
			if c.is_active() {
				for enemy in enemies.iter_mut() {
					let mut collisions: Vec<CollisionDecider> = Vec::with_capacity(5);
					if enemy.is_alive() {
						let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
						let pen = &mut 0.0;
						if enemy.rb.rect_vs_rect(c.rb, normal_collision, pen) && c.rb.vel.length() > 1.5 {
							enemy.projectile_knockback(c.x_vel(), c.y_vel());
						}
						if GameData::check_collision(&enemy.rb.pos(),&c.pos()){
							// crate squishes enemy
							match c.crate_type {
								CrateType::Explosive => {
									if c.rb.vel.length() > (c.killing_weight) * 7.0 {
										let scraps = c.explode(0);
										for scrap in scraps {
											explosion_shrapnel.push(scrap);
										}
										enemy.die();
									}
								}
								CrateType::Heavy => {
									if c.rb.vel.length() > (c.killing_weight) * 1.2 {
										enemy.die();
									}
								}
								_ => {
									if c.rb.vel.length() > (c.killing_weight) * 16.0 {
										if enemy.minus_hp(c.rb.vel.length() as i32) { self.game_data.boss_killed = true; }
									}
								}
							}
							if c.rb.vel.length() > (c.rb.i_mass * c.rb.friction) * 35.0 {
								enemy.die();
							}
							collisions.push(enemy.collect_col(enemy.rb.pos(), enemy.rb.hitbox.center_point(), c.pos()));
						}
					}
					enemy.resolve_col(&collisions);
				}
			}
		}

		// CRATE COLLISION vs CRATE COLLISION
		for i in 0 .. self.game_data.crates.len(){
			let (sp, other_crates) = self.game_data.crates.split_at_mut(i);
			let (source, after) = other_crates.split_first_mut().unwrap();
			for target in sp.iter_mut().chain(after.iter_mut()) {
				let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
				let pen = &mut 0.0;
				if source.is_active() && target.is_active(){
					if source.rb.rect_vs_rect(target.rb, normal_collision, pen) {
						source.rb.resolve_col(&mut target.rb, *normal_collision, *pen);
					}
				}
			}
		}

		// PLAYER PROJECTILE COLLISIONS VS ALL 
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				// PLAYER PROJECTILE vs ENEMY
				for enemy in enemies.iter_mut() {
					let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
					let pen = &mut 0.0;
					if enemy.is_alive() {
						if enemy.rb.rect_vs_circle(projectile.rb, normal_collision, pen) && projectile.is_active() {
							match enemy.enemy_type {
								EnemyType::Boss => {
									enemy.projectile_knockback(projectile.x_vel(), projectile.y_vel());
									if enemy.minus_hp(projectile.power.damage/3) { self.game_data.boss_killed = true; }
								}
								EnemyType::Skeleton=>{
									enemy.minus_hp(projectile.power.damage/2);
								}
								_ =>{
									enemy.projectile_knockback(projectile.x_vel(), projectile.y_vel());
									enemy.minus_hp(projectile.power.damage);
								}
							}
							projectile.die();
						}
					}
				}

				// PLAYER PROJECTILE vs CRATES
				for c in self.game_data.crates.iter_mut(){
					let normal_collision = &mut Vector2D{x : 0.0, y : 0.0};
					let pen = &mut 0.0;
					if c.is_active() {
						if c.rb.rect_vs_circle(projectile.rb, normal_collision, pen) && projectile.is_active() {
							if projectile.power.flammable && c.crate_type == CrateType::Explosive {
								// Explode
								let  scraps = c.explode(0);
								for scrap in scraps {
									explosion_shrapnel.push(scrap);
								}
							} else { c.rb.resolve_col(&mut projectile.rb, *normal_collision, *pen); }
							projectile.inc_bounce();
						}
					}
				}

				// PLAYER PROJECTILES vs ENEMY PROJECTILES
				for enemy_projectile in self.game_data.enemy_projectiles.iter_mut(){
					if enemy_projectile.is_active() {
						let normal_collision = &mut Vector2D{x : 0.0, y : 0.0};
						let pen = &mut 0.0;
						if projectile.rb.circle_vs_circle(enemy_projectile.rb, normal_collision, pen) && projectile.is_active() {
							projectile.rb.resolve_col(&mut enemy_projectile.rb, *normal_collision, *pen);
							projectile.inc_bounce();
							enemy_projectile.inc_bounce();
						}
					}
				}

				// SHRAPNEL vs PLAYER
				if projectile.is_shrapnel(){
					let normal_collision = &mut Vector2D{x : 0.0, y : 0.0};
					let pen = &mut 0.0;
					if player.rb.rect_vs_circle(projectile.rb, normal_collision, pen) && projectile.is_active() {
						player.rb.resolve_col(&mut projectile.rb, *normal_collision, *pen);
						player.minus_hp(6);
						projectile.die();
					}
				}
				projectile.check_bounce(&mut self.game_data.crates, map);
			}
		}

		// ENEMY PROJECTILE COLLISIONS
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			// ENEMY PROJECTILES vs PLAYER
			// TODO: POSSIBLY ADD PLAYER KNOCKBACK
			if check_collision(&projectile.pos(), &player.pos()) && projectile.is_active() {
				player.minus_hp(projectile.power.damage);
				projectile.die();
			}
			// ENEMY PROJECTILE vs CRATES
			for c in self.game_data.crates.iter_mut(){
				let normal_collision = &mut Vector2D{x : 0.0, y : 0.0};
				let pen = &mut 0.0;
				if c.is_active() {
					if c.rb.rect_vs_circle(projectile.rb, normal_collision, pen) && projectile.is_active() {
						if projectile.power.flammable && c.crate_type == CrateType::Explosive {
							// Explode
							let  scraps = c.explode(0);
							for scrap in scraps {
								explosion_shrapnel.push(scrap);
							}
						} else { c.rb.resolve_col(&mut projectile.rb, *normal_collision, *pen); }
						projectile.inc_bounce();
					}
				}
			}
			// ENEMY PROJECTILE vs CRATES + WALLS
			projectile.check_bounce(&mut self.game_data.crates, map);
		}	

		// COIN COLLECTION
		for coin in self.game_data.gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					player.add_coins(coin.get_gold());
				}
			}
		}
		for coin in self.game_data.blue_gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					self.game_data.blue_gold_count += 1; 
				}
			}
		}

		// PICKUPS
		let mut can_pickup = false;
		for drop in self.game_data.dropped_powers.iter_mut() {
			if check_collision(&player.pos(), &drop.pos()) {
				if !drop.collected() {
					match drop.power_type() {
						PowerType::None => {},
						_ => {
							can_pickup = true;
						}
					}
				}
			}
		}
		player.set_can_pickup(can_pickup);
		let mut can_pickup_shop = false;
		let mut price = 0;
		let mut i = 0; 
		while i < map_data.shop_spawns.len() {
			if map_data.shop_items[i].1 {
				i += 1;
				continue;
			}
			let pos = Rect::new((map_data.shop_spawns[i].1 as i32) * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								(map_data.shop_spawns[i].0 as i32) * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE, TILE_SIZE);
			if check_collision(&player.pos(), &pos) && player.get_pickup_timer() > 1000 {
				match map_data.shop_items[i].0 {
					ShopItems::None => { }
					_ => {
						can_pickup_shop = true;
						price = map_data.shop_items[i].2;
					}
				}
				break;
			}
			i += 1;
		}
		player.set_can_pickup_shop(can_pickup_shop);
		player.set_shop_price(price);
	}

	// draw player
	pub fn draw_player(&mut self, fps_avg: f64, player: &mut Player, curr_bg: Rect) {
		// draw player
		player.set_cam_pos(curr_bg.x(), curr_bg.y());
		player.set_hat_pos(curr_bg.x(), curr_bg.y());
		player.get_frame_display(&mut self.game_data, fps_avg);
		self.core.wincan.copy_ex(player.texture(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
		// draw shield outline on player
		if player.get_shielded() { 
			let texture_creator = self.core.wincan.texture_creator();
			let shield_outline = texture_creator.load_texture("images/abilities/shield_outline.png").unwrap();
			let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
			let pos = Rect::new(if player.facing_right { player.get_cam_pos().x-(TILE_SIZE_CAM/8) as i32 } else { player.get_cam_pos().x-(TILE_SIZE_CAM/4) as i32 }, 
								player.get_cam_pos().y-(TILE_SIZE_CAM/4) as i32, 
								TILE_SIZE_64+TILE_SIZE_CAM/4, 
								TILE_SIZE_64+TILE_SIZE_CAM/4);
			self.core.wincan.copy_ex(&shield_outline, src, pos, 0.0, None, !player.facing_right, false).unwrap(); 
		}
		let src = Rect::new(0, 0, TILE_SIZE_64, TILE_SIZE_64);
		if player.modifier.modifier_type != ModifierType::None {
			self.core.wincan.copy_ex(&player.mod_texture(), src, player.get_hat_pos(), 0.0, None, player.facing_right, false).unwrap(); 
		}
	}

	// draw player projectiles
	pub fn draw_player_projectile(&mut self, ability_textures: &Vec<Texture>, player: &Player, mousestate: MouseState)-> Result<(), String>  {
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				match projectile.power.power_type{
					PowerType::Slimeball=> {
						self.core.wincan.copy_ex(&ability_textures[0], projectile.src(), projectile.set_cam_pos(player), 0.0, None, !projectile.facing_right, false).unwrap();
					}
					PowerType::Rock=> {
                        self.core.wincan.copy_ex(&ability_textures[6], projectile.src(), projectile.set_cam_pos(player), projectile.angle, None, !projectile.facing_right, false).unwrap();
                    }
					PowerType::Fireball=> {
						let time = projectile.elapsed;
						
						//starting time, how many time for each frame, row of the pic, col of the pic, size of each frame
						let s = ROGUELIKE::display_animation(time, 4, 6, 6, TILE_SIZE);

						if mousestate.x() > player.get_cam_pos().x() && time == 0 {
							projectile.facing_right = true;
							if mousestate.y() < player.get_cam_pos().y() { //1st quadrant
								projectile.angle = projectile.angle- 2.0*projectile.angle;
							}
						} else if mousestate.x() < player.get_cam_pos().x()  && time == 0 {
							projectile.facing_right = false;
							if mousestate.y() < player.get_cam_pos().y() { //2nd quadrant
								projectile.angle = projectile.angle - 180.0;
							
							} else if mousestate.y() > player.get_cam_pos().y() { //third quadrant
								projectile.angle = projectile.angle- 2.0*projectile.angle - 180.0;
							}
						}
						projectile.elapsed += 1;
						if projectile.elapsed == 127 {projectile.die();}

						self.core.wincan.copy_ex(&ability_textures[1], s, projectile.set_cam_pos_large(player), projectile.angle, None, false, false).unwrap();
					}
					PowerType::Shield => {
						self.core.wincan.copy(&ability_textures[3], projectile.src(), projectile.set_cam_pos(player)).unwrap();
					}
					PowerType::Shrapnel => {
						self.core.wincan.copy_ex(&ability_textures[5], projectile.src(), projectile.set_cam_pos(player), projectile.angle, None, !projectile.facing_right, false).unwrap();				
					}
					_=>{}
				}	

			}
		}
		Ok(())
	}

	//draw player weapon
	pub fn draw_weapon(&mut self, player: &Player, sword_texture: &Texture, spear_texture: &Texture, dagger_texture: &Texture){
		let rotation_point;
		let pos;
		let mut angle = 0.0;
		let mut lunge = 0.0;

		// display weapon
		match player.get_weapon().weapon_type {
			WeaponType::Sword => {
				// weapon animation
				if player.get_attacking() {
					angle = (player.get_attack_timer() * 60 / 250 ) as f64 - 60.0;
				} else { angle = - 60.0; }
				// weapon position
				if player.facing_right{
					pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE_CAM as i32, 
									player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(0, (TILE_SIZE_HALF) as i32); //rotation center
				} else{
					pos = Rect::new(player.get_cam_pos().x() - player.get_weapon().attack_length as i32, 
									player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(player.get_weapon().attack_length as i32,  (TILE_SIZE_HALF)  as i32); //rotation center
					angle = -angle;
				}
				self.core.wincan.copy_ex(&sword_texture, None, pos, angle, rotation_point,
					player.facing_right, false).unwrap();
			},
			WeaponType::Spear => {
				// weapon animation
				if player.get_attacking() {
					if player.get_attack_timer() < player.get_weapon().attack_time/2 {
						lunge -= (TILE_SIZE_CAM*2/3) as f64 - (player.get_attack_timer() * 30 / 250 ) as f64;
					} else {
						lunge -= (TILE_SIZE_CAM*2/3) as f64 - (player.get_weapon().attack_time as f64 - player.get_attack_timer() as f64) * 30.0 / 250.0;
					}
				} else { lunge -= (TILE_SIZE_CAM*2/3) as f64 }
				// weapon position
				if player.facing_right{
					pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE_CAM as i32 + lunge as i32, 
									player.get_cam_pos().y() as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(0, (TILE_SIZE_HALF) as i32); //rotation center
				} else{
					pos = Rect::new(player.get_cam_pos().x() - player.get_weapon().attack_length as i32 - lunge as i32, 
									player.get_cam_pos().y() as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(player.get_weapon().attack_length as i32,  (TILE_SIZE_HALF)  as i32); //rotation center
					angle = -angle;
				}
				self.core.wincan.copy_ex(&spear_texture, None, pos, angle, rotation_point,
					player.facing_right, false).unwrap();
			},
			WeaponType::Dagger => {
				// weapon animation
				if player.get_attacking() {
					angle = (player.get_attack_timer() * 60 / 250 ) as f64 - 60.0;
				} else { angle = - 60.0; }
				// weapon position
				if player.facing_right{
					pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE_CAM as i32, 
									player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(0, (TILE_SIZE_HALF) as i32); //rotation center
				} else{
					pos = Rect::new(player.get_cam_pos().x() - player.get_weapon().attack_length as i32, 
									player.get_cam_pos().y()+(TILE_SIZE_CAM/2) as i32, 
									player.get_weapon().attack_length, TILE_SIZE_CAM * 7/5);
					rotation_point = Point::new(player.get_weapon().attack_length as i32,  (TILE_SIZE_HALF)  as i32); //rotation center
					angle = -angle;
				}
				self.core.wincan.copy_ex(&dagger_texture, None, pos, angle, rotation_point,
					player.facing_right, false).unwrap();
			},
			WeaponType::None => {}
		}
	}

	pub fn draw_enemy_projectile(&mut self,ability_textures: &Vec<Texture> , player: &Player) {
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active() {
				if matches!(projectile.power.power_type, PowerType::Slimeball) {
					self.core.wincan.copy(&ability_textures[2], projectile.src(), projectile.set_cam_pos(player)).unwrap();
				} else {
					self.core.wincan.copy(&ability_textures[6], projectile.src(), projectile.set_cam_pos(player)).unwrap();
				}
			}
		}
	}

	pub fn display_animation(start_time: u128, frames: i32, row: i32, col: i32, size: u32) -> Rect {
		let x = (start_time/frames as u128) as i32;
		let mut src_x = 0;
		let mut src_y = 0;

		for i in 0..row{
			if x < col*(i+1) {
				src_x = (x-i*col)*size as i32;
				src_y = i*size as i32;
				break
			}
		}
		Rect::new(src_x as i32, src_y as i32, size, size)
	}
}