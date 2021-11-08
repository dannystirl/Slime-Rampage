extern crate rogue_sdl;
use rogue_sdl::{Game, SDLCore};

use std::time::Duration;
use std::time::Instant;
//use std::cmp;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};//,TextureCreator};
//use sdl2::pixels::Color;
use rand::Rng;

mod background;
mod credits;
mod enemy;
mod gamedata;
mod gold;
mod player;
mod projectile;
mod room;
mod map;
mod ui;
mod crateobj;
mod rigidbody;

use crate::gamedata::*;
use crate::background::*;
use crate::player::*;
use crate::enemy::*;
use crate::projectile::*;
//use crate::gold::*;
//use crate::room::*;
//use crate::ui::*;
//use crate::crateobj::*;

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
}

// CREATE GAME
impl Game for ROGUELIKE  {

	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		let game_data = GameData::new();
		Ok(ROGUELIKE{ core, game_data })
	}

	fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();
		let mut rng = rand::thread_rng();

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		let mut player = player::Player::new(
			(CENTER_W as f64, CENTER_H as f64),
			texture_creator.load_texture("images/player/slime_sheet.png")?,
		);
		let mut ui = ui::UI::new(
			Rect::new(
				(10) as i32 *(TILE_SIZE as f64 *1.2) as i32,
				(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(TILE_SIZE as f64 *1.2) as u32,
				(TILE_SIZE as f64 *1.2) as u32,
			), 
			texture_creator.load_texture("images/ui/heart.png")?,
		);
		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		//let laser = texture_creator.load_texture("images/abilities/laser blast.png")?;
		//let fire_texture = texture_creator.load_texture("images/abilities/fireball.png")?;
		let bullet = texture_creator.load_texture("images/abilities/bullet.png")?; 
		let fireball = texture_creator.load_texture("images/abilities/beng.png")?; 

		let crate_texture = texture_creator.load_texture("images/objects/crate.png")?; 
		let mut bullet_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
		
		bullet_textures.push(bullet);
		bullet_textures.push(fireball);

		let mut crate_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
		crate_textures.push(crate_texture);
		let coin_texture = texture_creator.load_texture("images/ui/gold_coin.png")?;
		let sword = texture_creator.load_texture("images/player/sword_l.png")?;
		let mut crate_manager = crateobj::Crate::manager();

		// OBJECT GENERATION
		let pos = Rect::new(
		(CAM_W/2 - TILE_SIZE/2 -200 + rng.gen_range(1..10)) as i32,
		(CAM_H/2 - TILE_SIZE/2) as i32 -200 + rng.gen_range(0..10),
		TILE_SIZE,
		TILE_SIZE,);
		
		if !DEVELOP {
			self.game_data.crates.push(crateobj::Crate::new(pos));
		}

		// CREATE ROOM
		let background = background::Background::new(
			texture_creator.load_texture("images/background/bb.png")?,
			texture_creator.load_texture("images/background/floor_tile_1.png")?,
			texture_creator.load_texture("images/background/floor_tile_2.png")?,
			texture_creator.load_texture("images/background/tile.png")?,
			texture_creator.load_texture("images/background/skull.png")?,
			texture_creator.load_texture("images/background/upstairs.png")?,
			texture_creator.load_texture("images/background/downstairs.png")?,
			self.game_data.rooms[self.game_data.current_room].xwalls,
			self.game_data.rooms[self.game_data.current_room].ywalls,
			Rect::new(
				(player.x() as i32 + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32),
				(player.y() as i32 + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32),
				CAM_W,
				CAM_H,
			),
		);
		let mut map_data = map::Map::new(
			background, 
		);
		map_data.create_map();

		// set starting position
		player.set_x((map_data.starting_position.0 * TILE_SIZE as i32 - (CAM_W as i32 - TILE_SIZE as i32) / 2) as f64);
		player.set_y((map_data.starting_position.1 * TILE_SIZE as i32 - (CAM_H as i32 - TILE_SIZE as i32) / 2) as f64);

		let mut enemies: Vec<Enemy> = Vec::new();	// Size is max number of enemies
		let mut rngt = Vec::new();

		/* let ghost_tex = texture_creator.load_texture("images/enemies/place_holder_enemy.png")?;
		let gellem_tex = texture_creator.load_texture("images/enemies/ranged_enemy.png")?; */

		let mut enemy_count = 0;
		for h in 0..MAP_SIZE_H {
			for w in 0..MAP_SIZE_W {
				if map_data.enemy_spawns[h][w] == 0 {
					continue;
				}
				if DEBUG { println!("{}, {}", w, h); }
				match map_data.enemy_spawns[h][w] {
					1 => {
						let e = enemy::Enemy::new(
							Rect::new(
								w as i32 * TILE_SIZE as i32/*  - (player.x() % TILE_SIZE as f64) as i32 */ - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								h as i32 * TILE_SIZE as i32/*  - (player.y() % TILE_SIZE as f64) as i32 */ - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE / 2,
								TILE_SIZE / 2
							),
							texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
							EnemyType::Melee,
							enemy_count,
						);
						enemies.push(e);
						rngt.push(rng.gen_range(1..5));
						enemy_count += 1;
					}
					2 => {
						let e = enemy::Enemy::new(
							Rect::new(
								w as i32 * TILE_SIZE as i32/*  - (player.x() % TILE_SIZE as f64) as i32 */ - (CAM_W as i32 - TILE_SIZE as i32) / 2,
								h as i32 * TILE_SIZE as i32/*  - (player.y() % TILE_SIZE as f64) as i32 */ - (CAM_H as i32 - TILE_SIZE as i32) / 2,
								TILE_SIZE / 2,
								TILE_SIZE / 2
							),
							texture_creator.load_texture("images/enemies/ranged_enemy.png")?,
							EnemyType::Ranged,
							enemy_count,
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

		// MAIN GAME LOOP
		'gameloop: loop {
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
			player.set_x_delta(0);
			player.set_y_delta(0);
			self.core.wincan.copy(&map_data.background.black, None, None)?;

			// GET INPUT
			let mousestate= self.core.event_pump.mouse_state();
			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();
			ROGUELIKE::check_inputs(self, &keystate, mousestate, &mut player)?;

			// UPDATE BACKGROUND
			ROGUELIKE::draw_background(self, &player, &mut map_data.background, map_data.map)?;

			// UPDATE PLAYER
			player.update_player(&self.game_data, map_data.map, &mut self.core)?;
			ROGUELIKE::draw_player(self, fps_avg, &mut player, map_data.background.get_curr_background());

			// UPDATE ENEMIES
			if elapsed > Duration::from_secs(2) {
				rngt = ROGUELIKE::update_enemies(self, &mut rngt, &mut enemies, &player,map_data.map);
			}
		
			// UPDATE ATTACKS
			// Should be switched to take in array of active fireballs, bullets, etc.
			ROGUELIKE::update_projectiles(&mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
			ROGUELIKE::draw_enemy_projectile(self, &bullet_textures, &player);	
			ROGUELIKE::draw_player_projectile(self,  &bullet_textures,  &player)?;	
			ROGUELIKE::draw_weapon(self, &player,&sword);
			
			// UPDATE INTERACTABLES
			// function to check explosive barrels stuff like that should go here. placed for ordering.
			ROGUELIKE::update_gold(self, &mut enemies, &mut player, &coin_texture);
			crate_manager.update_crates(&mut self.game_data, &mut self.core, &crate_textures, &player);
			for c in self.game_data.crates.iter_mut() {
				self.core.wincan.copy(&crate_textures[0],c.src(),c.offset_pos(&player))?;
			}

			// CHECK COLLISIONS
			ROGUELIKE::check_collisions(self, &mut player, &mut enemies, map_data.map);
			if player.is_dead(){break 'gameloop;}

			// UPDATE UI
			ui.update_ui( &player, &mut self.core)?;
			
			// UPDATE FRAME
			self.core.wincan.present();
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
	pub fn draw_background(&mut self, player: &Player, background: &mut Background, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let floor = texture_creator.load_texture("images/background/floor_tile_1.png")?;
		let tile = texture_creator.load_texture("images/background/tile.png")?;
		let upstairs = texture_creator.load_texture("images/background/upstairs.png")?;
		let downstairs = texture_creator.load_texture("images/background/downstairs.png")?;

		background.set_curr_background(player.x(), player.y(), player.width(), player.height());

		let h_bounds_offset = (player.y() / TILE_SIZE as f64) as i32;
		let w_bounds_offset = (player.x() / TILE_SIZE as f64) as i32;
	
		if DEVELOP {
			for h in 0..(CAM_H / TILE_SIZE) + 1 {
				for w in 0..(CAM_W / TILE_SIZE) + 1 {
					let src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
					let pos = Rect::new((w as i32 + 0 as i32) * TILE_SIZE as i32 - (player.x() % TILE_SIZE as f64) as i32 /* + (CENTER_W - player.x() as i32) */,
						(h as i32 + 0 as i32) * TILE_SIZE as i32 - (player.y() % TILE_SIZE as f64) as i32 /* + (CENTER_H - player.y() as i32) */,
						TILE_SIZE, TILE_SIZE);
					if h as i32 + h_bounds_offset < 0 ||
					   w as i32 + w_bounds_offset < 0 ||
					   h as i32 + h_bounds_offset >= MAP_SIZE_H as i32 ||
					   w as i32 + w_bounds_offset >= MAP_SIZE_W as i32 ||
					   map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 0 {
						continue;
					} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 1 {
						self.core.wincan.copy_ex(&floor, src, pos, 0.0, None, false, false).unwrap();
					} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 2 {
						self.core.wincan.copy_ex(&tile, src, pos, 0.0, None, false, false).unwrap();
					} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 3 {
						self.core.wincan.copy_ex(&upstairs, src, pos, 0.0, None, false, false).unwrap();
					} else if map[(h as i32 + h_bounds_offset) as usize][(w as i32 + w_bounds_offset) as usize] == 4 {
						self.core.wincan.copy_ex(&downstairs, src, pos, 0.0, None, false, false).unwrap();
					}
				}
			}
		} else {
			let tiles = &self.game_data.rooms[self.game_data.current_room].tiles;
			let mut n = 0;
			for i in 0..self.game_data.rooms[0].xwalls.1+1 {
				for j in 0..self.game_data.rooms[0].ywalls.1+1 {
					if tiles[n].0 {
						let t = background.get_tile_info(tiles[n].1, i, j, player.x(), player.y());
						self.core.wincan.copy_ex(t.0, t.1, t.2, 0.0, None, false, false).unwrap();
					}
					n+=1;
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
				if self.game_data.frame_counter.elapsed().as_millis() % 120 as u128 == 0 as u128 /* || 
				   enemy.force_move(&self.game_data) */ { // keep comment. this check will stop enemies from running into walls
					rngt[i] = rand::thread_rng().gen_range(1..5);
				}
				let t = enemy.update_enemy(&self.game_data, rngt, i, (player.x(), player.y()), map);
				self.core.wincan.copy(enemy.txtre(), enemy.src(), t).unwrap();
				i += 1;
			}
		}
		return rngt.to_vec();
	}

	pub fn update_gold(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, coin_texture: &Texture) {
		//add coins to gold vector
		for enemy in enemies {
			if !enemy.is_alive() && enemy.has_gold(){	// Should be changed to has_drop() when more drops
				let drop = enemy.drop_item();
				self.game_data.gold.push(drop);
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
	}

	// check input values
	pub fn check_inputs(&mut self, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player)-> Result<(), String>  {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - self.game_data.get_accel_rate() as i32);
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - self.game_data.get_accel_rate() as i32);
			player.facing_right = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + self.game_data.get_accel_rate() as i32);
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + self.game_data.get_accel_rate() as i32);
			player.facing_right = true;
		}
		// basic attack
		if keystate.contains(&Keycode::Space) {
			if !(player.is_attacking) {
				player.attack();
			}
		}
		// Shoot ranged attack
		if mousestate.left(){
			if !player.is_firing && player.get_mana() > 0 {
				let p_type = ProjectileType::Bullet;
				
				let b = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(),p_type);
				self.game_data.player_projectiles.push(b);
			}
		}
		//ability
		if keystate.contains(&Keycode::F){
			if !player.is_firing && player.get_mana() > 0 {
				let p_type = ProjectileType::Fireball;
				let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit(), p_type);
				self.game_data.player_projectiles.push(bullet);
			}
			//println!("you found the easter egg");
		}
		// FOR TESTING ONLY: USE TO FOR PRINT VALUES
		if keystate.contains(&Keycode::P) {
			//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
			//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
			println!("{} {}", player.x(), player.y());	
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
	fn check_collisions(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>, map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) {
		let xbounds = self.game_data.rooms[0].xbounds;
		let ybounds = self.game_data.rooms[0].ybounds;
		/* let bounds1 = Rect::new(xbounds.0, ybounds.0, TILE_SIZE, TILE_SIZE);
		let bounds2 = Rect::new(xbounds.1, ybounds.1, TILE_SIZE, TILE_SIZE); */

		for enemy in enemies {
			if !enemy.is_alive() {
				continue;
			}

			// player collision
			if check_collision(&player.pos(), &enemy.pos()) {
				player.minus_hp(5);
				player.set_invincible();
			}

			// player projectile collisions
			for projectile in self.game_data.player_projectiles.iter_mut(){
				if check_collision(&projectile.pos(), &enemy.pos())  && projectile.is_active() {
					enemy.knockback(projectile.x().into(), projectile.y().into(), xbounds, ybounds);
					enemy.minus_hp(5);
					projectile.die();
				}
				
			}

			// player melee collisions
			if player.is_attacking {
				if check_collision(&player.get_attack_box(), &enemy.pos()) {
					enemy.knockback(player.x().into(), player.y().into(), xbounds, ybounds);
					enemy.minus_hp(1);
				}
			}
		
			// enemy projectile collisions
			for projectile in self.game_data.enemy_projectiles.iter_mut(){
				if check_collision(&projectile.pos(), &player.pos()) && projectile.is_active() {
					player.minus_hp(5);
					player.set_invincible();
					projectile.die();
				}
			}
		}

		for projectile in self.game_data.player_projectiles.iter_mut(){
			projectile.check_bounce( xbounds, ybounds, map);
		}
		for projectile in self.game_data.enemy_projectiles.iter_mut(){
			projectile.check_bounce( xbounds, ybounds, map);
		}
		for coin in self.game_data.gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					player.add_coins(coin.get_gold());
				}
			}
		}
		for c in self.game_data.crates.iter_mut(){
			if check_collision(&player.pos(), &c.pos()){
				// provide impulse
				c.update_velocity(player.x_vel() as f64 * player.get_mass(), player.y_vel() as f64 * player.get_mass());
				//player.set_x_vel(0);
				//player.set_y_vel(0);
			} else {
				c.friction();
			}
		}
	}

	// draw player
	pub fn draw_player(&mut self, fps_avg: f64, player: &mut Player, curr_bg: Rect) {
		player.set_cam_pos(curr_bg.x(), curr_bg.y());
		player.get_frame_display(&mut self.game_data, fps_avg);
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
	}

	// draw player projectiles
	pub fn draw_player_projectile(&mut self, bullet_textures: &Vec<Texture>, player: &Player)-> Result<(), String>  {
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				match projectile.p_type{
					ProjectileType::Bullet=>{
						self.core.wincan.copy(&bullet_textures[0], projectile.src(), projectile.offset_pos(player)).unwrap();
					}
					ProjectileType::Fireball=>{
						self.core.wincan.copy(&bullet_textures[1], projectile.src(), projectile.offset_pos(player)).unwrap();

					}
				}	
			}
		}
		Ok(())
	}

	//draw player weapon
	pub fn draw_weapon(&mut self, player: &Player, texture : &Texture){
		let rotation_point;
		let pos;
		let mut angle;

		// weapon animation
		if player.is_attacking {
			angle = (player.get_attack_timer() * 60 / 250 ) as f64 - 60.0;
		} else { angle = - 60.0; }
		// display weapon
		if player.facing_right{
			pos = Rect::new(player.get_cam_pos().x() + TILE_SIZE as i32, player.get_cam_pos().y()+(TILE_SIZE/2) as i32, ATTACK_LENGTH, TILE_SIZE);
			rotation_point = Point::new(0, (TILE_SIZE/2) as i32); //rotation center
		} else{
			pos = Rect::new(player.get_cam_pos().x() - ATTACK_LENGTH as i32, player.get_cam_pos().y()+(TILE_SIZE/2) as i32, ATTACK_LENGTH, TILE_SIZE);
			rotation_point = Point::new(ATTACK_LENGTH as i32,  (TILE_SIZE/2)  as i32); //rotation center
			angle = -angle;
		}
		self.core.wincan.copy_ex(&texture, None, pos, angle, rotation_point, player.facing_right, false).unwrap();
	}

	pub fn draw_enemy_projectile(&mut self,bullet_textures: &Vec<Texture> , player: &Player) {
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active(){
				self.core.wincan.copy(&bullet_textures[0], projectile.src(), projectile.offset_pos(player)).unwrap();
			}
		}
	}
}