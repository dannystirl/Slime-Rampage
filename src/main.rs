extern crate rogue_sdl;
use rogue_sdl::{Game, SDLCore};
use vector::Vector2D;
//use sdl2::audio::AudioSpecDesired;
//use sdl2::audio::AudioSpecWAV;
//use sdl2::audio::AudioCVT;
//use sdl2::audio::AudioCallback;
use std::time::Duration;
use std::time::Instant;
//use std::cmp;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};
use rand::Rng;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
//use std::env;
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
mod room;
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

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
}

// CREATE GAME
impl Game for ROGUELIKE  {

	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		let game_data = GameData::new();
		Ok(ROGUELIKE{ core, game_data, })
	}

	fn run(&mut self) -> Result<(), String> {
		// CREATE GAME CONSTANTS
        let texture_creator = self.core.wincan.texture_creator();
		//let rng = rand::thread_rng();

		// AUDIO SYSTEM
		let frequency = 44_100;
		let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
		let channels = DEFAULT_CHANNELS; // Stereo
		let chunk_size = 1_024;
		sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
		let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
		// Number of mixing channels available for sound effect `Chunk`s to play simultaneously.
		sdl2::mixer::allocate_channels(4);
	
		if DEBUG {
			let n = sdl2::mixer::get_chunk_decoders_number();
			println!("available chunk(sample) decoders: {}", n);
			for i in 0..n {
				println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
			}
			println!("available music decoders: {}", n);
			let n = sdl2::mixer::get_music_decoders_number();
			for i in 0..n {
				println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
			}
			println!("query spec => {:?}", sdl2::mixer::query_spec());
		}

		let path = Path::new("./music/Rampage.wav");
		let music = sdl2::mixer::Music::from_file(path)?;
		music.play(1)?;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		// create player 
		let class = PlayerType::Classic; 
		let mut player: Player; 
		match class {
			PlayerType::Warrior => {
				player = player::Player::new(
					texture_creator.load_texture("images/player/green_slime_sheet.png").unwrap(), 
					class, 
				); 
			}, 
			PlayerType::Assassin => {
				player = player::Player::new(
					texture_creator.load_texture("images/player/pink_slime_sheet.png").unwrap(), 
					class, 
				); 
			}, 
			_ => {
				player = player::Player::new(
					texture_creator.load_texture("images/player/blue_slime_sheet.png").unwrap(), 
					class, 
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
		let fireball_texture = texture_creator.load_texture("images/abilities/fireball_pickup.png")?;
		let slimeball_texture = texture_creator.load_texture("images/abilities/bullet_pickup.png")?;
		let shield_texture = texture_creator.load_texture("images/abilities/shield_pickup.png")?;
		let dash_texture = texture_creator.load_texture("images/abilities/dash_pickup.png")?;
		let sword_texture = texture_creator.load_texture("images/weapons/sword.png")?;
		let spear_texture = texture_creator.load_texture("images/weapons/spear.png")?;
		let health_texture = texture_creator.load_texture("images/ui/heart.png")?; 
		let health_upgrade_texture = texture_creator.load_texture("images/ui/heart_upgrade.png")?;
		let rock_texture = texture_creator.load_texture("images/abilities/rock.png")?; //need to change it to a new texture

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
			if self.game_data.current_floor > 3 {
				map_data.create_boss();
			} else {
				map_data.create_map();
			}

			// set starting position
			player.set_x((map_data.starting_position.0 as i32 * TILE_SIZE as i32 - (CAM_W - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);
			player.set_y((map_data.starting_position.1 as i32 * TILE_SIZE as i32 - (CAM_H - 2*TILE_SIZE_PLAYER) as i32 / 2) as f64);

			// reset arrays
			self.game_data.crates = Vec::<Crate>::with_capacity(0);
			self.game_data.dropped_powers = Vec::<Power>::with_capacity(0);
			self.game_data.dropped_weapons = Vec::<Weapon>::with_capacity(0);
			self.game_data.gold = Vec::<Gold>::with_capacity(0);
			self.game_data.player_projectiles = Vec::<Projectile>::with_capacity(0);
			self.game_data.enemy_projectiles = Vec::<Projectile>::with_capacity(0);
			// OBJECT GENERATION
			if DEVELOP {
				let pos = Rect::new(
					player.x() as i32 -200 + rng.gen_range(1..10),
					player.y() as i32 -200 + rng.gen_range(0..10),
					TILE_SIZE,
					TILE_SIZE
				);
				self.game_data.crates.push(crateobj::Crate::new(pos));
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
								EnemyType::Ranged,
								enemy_count,
								self.game_data.current_floor, 
							);
							enemies.push(e);
							rngt.push(rng.gen_range(1..5));
							enemy_count += 1;
						}
						3 => {
							let roll= rng.gen_range(0..10);
							//let roll = 1;
							if roll == 0 {
								let c = crateobj::Crate::new_heavy(
									Rect::new(
										w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
										h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
										62,
										62
									)
								);
								self.game_data.crates.push(c);
							}else if roll == 1 {
								let c = crateobj::Crate::new_explosive(
									Rect::new(
										w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
										h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
										//TILE_SIZE_64,
										TILE_SIZE_PLAYER*2,
										TILE_SIZE_PLAYER*2
									)
								);
								self.game_data.crates.push(c);
							}else{
								let c = crateobj::Crate::new(
									Rect::new(
										w as i32 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) /2,
										h as i32 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) /2,
										TILE_SIZE_PLAYER,
										TILE_SIZE_PLAYER
									)
								);
								self.game_data.crates.push(c);
							}
							
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
                                    TILE_SIZE_CAM * 4,
                                    TILE_SIZE_CAM * 4
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

			let mut all_frames = 0;
			let last_time = Instant::now();

			// INDIVIDUAL LEVEL LOOP
			'level: loop {
				for event in self.core.event_pump.poll_iter() {
					match event {
						Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
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
				ROGUELIKE::draw_weapon(self, &player, &sword_texture, &spear_texture);
				
				// UPDATE INTERACTABLES

				ROGUELIKE::update_drops(self, &mut enemies, &mut player, &mut map_data, &coin_texture,
										&fireball_texture, &slimeball_texture, &shield_texture,
										&dash_texture, &health_texture, &health_upgrade_texture,
										&sword_texture, &spear_texture, &rock_texture,);

				// CHECK COLLISIONS
				ROGUELIKE::check_collisions(self, &mut player, &mut enemies, &mut map_data, &crate_textures, fps_avg, &mut explosion_shrapnel);
				if player.is_dead(){break 'gameloop;}

				// Check if any shrapnel has been added and append
				if explosion_shrapnel.len() > 0{
					for scrap in explosion_shrapnel{
						self.game_data.player_projectiles.push(scrap);
					}
				}

				// UPDATE UI
				ui.update_ui(&player, &mut self.core, &map_data, &self.game_data)?;
				
				// UPDATE FRAME
				self.core.wincan.present();
			}
			self.game_data.current_floor += 1;
        	self.game_data.map_size_w = 61 + ((self.game_data.current_floor-1)*30) as usize;
        	self.game_data.map_size_h = 61 + ((self.game_data.current_floor-1)*30) as usize;
		}
		// Out of game loop, return Ok
		Ok(()) 
	}
}

pub fn main() -> Result<(), String> {
    rogue_sdl::runner(TITLE, ROGUELIKE::init);
	//credits::run_credits()
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
	// draw background
//	pub fn draw_everything(&mut self,player: &Player, enemies: Vec<Enemy>){
		

//	}
	pub fn draw_background(&mut self, player: &Player, background: &mut Background, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H], dirt_map: [[(i32,i32); MAP_SIZE_W]; MAP_SIZE_H]) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let _floor = texture_creator.load_texture("images/background/floor_tile_1.png")?;
		let dirt_sheet = texture_creator.load_texture("images/background/dirt_sheet.png")?;
		let shop = texture_creator.load_texture("images/background/floor_tile_maroon.png")?;
		let tile = texture_creator.load_texture("images/background/tile.png")?;
		let moss_tile = texture_creator.load_texture("images/background/moss_tile.png")?;
		let upstairs = texture_creator.load_texture("images/background/upstairs.png")?;
		let downstairs = texture_creator.load_texture("images/background/downstairs.png")?;
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
						_ => { self.core.wincan.copy_ex(&downstairs, src, pos, 0.0, None, false, false).unwrap(); },  	// downstairs tile
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
				let t = enemy.update_enemy(&self.game_data, rngt, i, (player.x(), player.y()), map);
				self.core.wincan.copy_ex(enemy.txtre(), enemy.src(), t, 0.0, None, enemy.facing_right, false).unwrap();
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
	
	pub fn update_drops(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, map_data: &mut Map, coin_texture: &Texture,
						fireball_texture: &Texture, slimeball_texture: &Texture, shield_texture: &Texture,
						dash_texture: &Texture, health_texture: &Texture, health_upgrade_texture: &Texture,
						sword_texture: &Texture, spear_texture: &Texture, rock_texture: &Texture,) {
		//add enemy drops to game
		for enemy in enemies {
			if !enemy.is_alive() && enemy.has_item() {
				if enemy.has_coin() {
					self.game_data.gold.push(enemy.drop_coin());
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

		// draw weapons
		for weapon in self.game_data.dropped_weapons.iter_mut() {
			let pos = Rect::new(weapon.x() as i32 + (CENTER_W - player.x() as i32),
								weapon.y() as i32 + (CENTER_H - player.y() as i32),
								TILE_SIZE_POWER, TILE_SIZE_POWER);
			match weapon.weapon_type() {
				WeaponType::Sword => {
					self.core.wincan.copy_ex(&sword_texture, weapon.src(), pos, 0.0, None, false, false).unwrap();
				},
				WeaponType::Spear => {
					self.core.wincan.copy_ex(&spear_texture, weapon.src(), pos, 0.0, None, false, false).unwrap();
				},
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
				ShopItems::HealthUpgrade => {
					self.core.wincan.copy_ex(&health_upgrade_texture, src, pos, 0.0, None, false, false).unwrap();
				}
				ShopItems::Health => {
					self.core.wincan.copy_ex(&health_texture, src, pos, 0.0, None, false, false).unwrap();
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
						// code for placeable shield. 
						//let bullet = player.fire(player.x() as i32, player.y() as i32, 0.0, PowerType::Shield, 0);
						//self.game_data.player_projectiles.push(bullet);
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
											let damage = player.get_weapon_damage(); 
											player.set_weapon_damage(damage+2);
										}
									}
								}
								ShopItems::Spear => {
									if map_data.shop_items[i].1 == false {
										if player.get_weapon().weapon_type != WeaponType::Spear {
											player.set_weapon(WeaponType::Spear)
										} else {
											let damage = player.get_weapon_damage(); 
											player.set_weapon_damage(damage+4);
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
								_ => { }
							}
							map_data.shop_items[i].1 = true;
							picked_up = true;
							break;
						}
						i+=1; 
					}
				}
				if !picked_up {
					for drop in self.game_data.dropped_weapons.iter_mut() {
						if check_collision(&player.pos(), &drop.pos()) && player.get_pickup_timer() > 1000 {
							player.reset_pickup_timer();
							let weapon_type = drop.weapon_type; 
							if player.get_weapon().weapon_type != weapon_type {
								player.set_weapon(weapon_type)
							} else {
								let damage = player.get_weapon_damage(); 
								player.set_weapon_damage(damage+15);
							}
							drop.set_weapon_type(weapon_type);
							break;
						}
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
			//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
			//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
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
						enemy.minus_hp(player.get_weapon().damage);
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
					//println!("player hits crate");
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
					if enemy.is_alive() {
						let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
						let pen = &mut 0.0;
						if enemy.rb.rect_vs_rect(c.rb, normal_collision, pen) && c.rb.vel.length() > 1.5 {//rb collision. rect vs rect
							enemy.projectile_knockback(c.x_vel(), c.y_vel());
						}
					}
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
							if enemy.rb.rect_vs_circle(projectile.rb, normal_collision, pen) {
								match enemy.enemy_type {
									EnemyType::Boss => {
										enemy.projectile_knockback(projectile.x_vel(), projectile.y_vel());
										enemy.minus_hp(projectile.power.damage/3);
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
						if c.rb.rect_vs_circle(projectile.rb, normal_collision, pen) {
							if projectile.is_flammable() {
								// Explode
								let  scraps = c.explode(0);
								for scrap in scraps {
									explosion_shrapnel.push(scrap);
								}
								println!("KABOOOOOOOM!!!!!!!!");
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
						if projectile.rb.circle_vs_circle(enemy_projectile.rb, normal_collision, pen){
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
					if player.rb.rect_vs_circle(projectile.rb, normal_collision, pen) {
						player.rb.resolve_col(&mut projectile.rb, *normal_collision, *pen);
						player.minus_hp(10);
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
				player.minus_hp(projectile.power.damage as u32);
				projectile.die();
			}
			for c in self.game_data.crates.iter_mut(){
				if c.is_active() {
					let normal_collision = &mut Vector2D { x: 0.0, y: 0.0 };
					let pen = &mut 0.0;
					if c.rb.rect_vs_circle(projectile.rb, normal_collision, pen) {
						c.rb.resolve_col(&mut projectile.rb, *normal_collision, *pen);
						projectile.inc_bounce();
					}
				}
			}

			// ENEMY PROJECTILE vs CRATES + WALLS
			projectile.check_bounce(&mut self.game_data.crates, map);
		}
		// ENEMY PROJECTILE vs CRATES
	

		// COINS
		for coin in self.game_data.gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					player.add_coins(coin.get_gold());
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
		let mut can_pickup_weapon = false;
		for drop in self.game_data.dropped_weapons.iter_mut() {
			if check_collision(&player.pos(), &drop.pos()) {
				can_pickup_weapon = true;
			}
		}
		player.set_can_pickup_weapon(can_pickup_weapon);
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

						if mousestate.x() > player.get_cam_pos().x() && time == 0{
							projectile.facing_right = true;
						}else if mousestate.x() < player.get_cam_pos().x()  && time == 0{
							projectile.facing_right = false;
						}
						if mousestate.y() > player.get_cam_pos().y() && time == 0{
							projectile.facing_up = false;
						}else if mousestate.y() < player.get_cam_pos().y()  && time == 0{
							projectile.facing_up = true;
						}
					
						projectile.elapsed += 1;
						if projectile.elapsed == 127 {projectile.die();}

						self.core.wincan.copy_ex(&ability_textures[1], s, projectile.set_cam_pos_large(player), projectile.angle, None, !projectile.facing_right, false).unwrap();
					}
					PowerType::Shield => {
						self.core.wincan.copy(&ability_textures[3], projectile.src(), projectile.set_cam_pos(player)).unwrap();
					}
					PowerType::Shrapnel => {
						self.core.wincan.copy_ex(&ability_textures[5], projectile.src(), projectile.set_cam_pos(player), 0.0, None, !projectile.facing_right, false).unwrap();				
					}
					_=>{}
				}	

			}
		}
		Ok(())
	}

	//draw player weapon
	pub fn draw_weapon(&mut self, player: &Player, sword_texture: &Texture, spear_texture: &Texture){
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