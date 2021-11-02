extern crate rogue_sdl;
mod background;
mod credits;
mod enemy;
mod gamedata;
mod gold;
mod player;
mod projectile;
mod room;
mod ui;
mod crateobj;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use rogue_sdl::{Game, SDLCore};
use crate::background::*;
use crate::enemy::*;
use crate::gamedata::*;
//use crate::gold::*;
use crate::player::*;
use crate::projectile::*;
//use crate::room::*;
//use crate::ui::*;
//use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture};//,TextureCreator};
use crate::crateobj::*;

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

		let mut count = 0;
		let f_display = 15;

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
		let laser = texture_creator.load_texture("images/abilities/laser blast.png")?;
		//let fire_texture = texture_creator.load_texture("images/abilities/fireball.png")?;
		let bullet = texture_creator.load_texture("images/abilities/bullet.png")?; 
		let crate_texture = texture_creator.load_texture("images/objects/crate.png")?; 
		let mut crate_textures: Vec<Texture> = Vec::<Texture>::with_capacity(5);
		crate_textures.push(crate_texture);
		let coin_texture = texture_creator.load_texture("images/ui/gold_coin.png")?;
		let sword = texture_creator.load_texture("images/player/sword_l.png")?;
		let mut crate_manager = crateobj::Crate::manager();
		//crate generation
		let mut rng = rand::thread_rng();
		let num = rng.gen_range(1..500);

		let pos = Rect::new(
		(CAM_W/2 - TILE_SIZE/2 -100 + rng.gen_range(1..200)) as i32,
		(CAM_H/2 - TILE_SIZE/2) as i32 -100 + rng.gen_range(10..100),
		TILE_SIZE,
		TILE_SIZE,);
		self.game_data.crates.push(crateobj::Crate::new(pos));
		//crate generation over

		let mut enemies: Vec<Enemy> = Vec::with_capacity(5);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement. if we make an entities file, this should probably be moved there
		let mut i=1;
		for _ in 0..enemies.capacity(){
			let num = rng.gen_range(1..5);
			let enemy_type: EnemyType; 
			match num {
				5 => { enemy_type = EnemyType::Ranged } 
				4 => { enemy_type=  EnemyType::Ranged } 
				_ => { enemy_type = EnemyType::Melee } 
			}

			match enemy_type{
			EnemyType::Ranged=>{
				let e = enemy::Enemy::new(
					Rect::new(
						(CAM_W/2 - TILE_SIZE/2 + 200 + 5*rng.gen_range(5..20)) as i32,
						(CAM_H/2 - TILE_SIZE/2) as i32 + 5*rng.gen_range(5..20),
						TILE_SIZE,
						TILE_SIZE,
					),
					texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
					enemy_type,
					i,
				);
				enemies.push(e);
			}
			EnemyType::Melee=>
			{
				let e = enemy::Enemy::new(
					Rect::new(
						(CAM_W/2 - TILE_SIZE/2 + 200 + 5*rng.gen_range(5..20)) as i32,
						(CAM_H/2 - TILE_SIZE/2) as i32 + 5*rng.gen_range(5..20),
						TILE_SIZE,
						TILE_SIZE,
					),
					texture_creator.load_texture("images/enemies/ranged_enemy.png")?,
					enemy_type,
					i,
				);
				enemies.push(e);
			}
		}

			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}
		// CREATE ROOM
		let mut background = background::Background::new(
			texture_creator.load_texture("images/background/bb.png")?,
			texture_creator.load_texture("images/background/floor_tile_1.png")?,
			texture_creator.load_texture("images/background/floor_tile_2.png")?,
			texture_creator.load_texture("images/background/tile.png")?,
			texture_creator.load_texture("images/background/skull.png")?,
			self.game_data.rooms[self.game_data.current_room].xwalls,
			self.game_data.rooms[self.game_data.current_room].ywalls,
			Rect::new(
				(player.x() as i32 + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32),
				(player.y() as i32 + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32),
				CAM_W,
				CAM_H,
			),
		);

		let mut map = ROGUELIKE::create_map();

		// WARNING! WIDTH AND HEIGHT ARE SWITCHED!
		println!("");
		for i in 0..MAP_SIZE_W {
			for j in 0..MAP_SIZE_H {
				print!("{} ", map[j][i]);
			}
			println!("");
		}

		let mut all_frames = 0;
		let last_time = Instant::now();

		let mut p_frames = 0;
		let mut src_y = 0;
		let mut count_y = 0;
		// MAIN GAME LOOP
		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}
			// fps calculations
			all_frames += 1;
			let elapsed = last_time.elapsed();
			if elapsed > Duration::from_secs(1) {
				let mut fps_avg = (all_frames as f64) / elapsed.as_secs_f64();
				//println!("Average FPS: {:.2}", fps_avg);
				fps_avg = fps_avg.recip();
				self.game_data.set_speed_limit(fps_avg * SPEED_LIMIT);
				self.game_data.set_accel_rate(fps_avg * ACCEL_RATE);

			}
			// reset frame values
			player.set_x_delta(0);
			player.set_y_delta(0);
			self.core.wincan.copy(&background.black, None, None)?;

			// GET INPUT
			let mousestate= self.core.event_pump.mouse_state();
			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();
			ROGUELIKE::check_inputs(self, &keystate, mousestate, &mut player);

			// UPDATE BACKGROUND
			ROGUELIKE::update_background(self, &player, &mut background)?;

			// UPDATE PLAYER
			player.update_player(&self.game_data);
			self.draw_player(&count, &f_display, &mut player, background.get_curr_background());
			count = count + 1;
			if count > f_display * 5 {
				count = 0;
			}

			// UPDATE ENEMIES
			if elapsed > Duration::from_secs(2) {
				rngt = ROGUELIKE::update_enemies(self, &mut rngt, &mut enemies, &player);
			}

			//UPDATE INTERACTABLES (GOLD FOR NOW)
			ROGUELIKE::update_gold(self, &mut enemies, &mut player, &coin_texture)?;
		
			// UPDATE ATTACKS
			// Should be switched to take in array of active fireballs, bullets, etc.
			ROGUELIKE::update_projectiles(&mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
			crate_manager.update_crates(&mut self.game_data, &mut self.core, &crate_textures,&player);
			ROGUELIKE::draw_enemy_projectile(self, &bullet, &player);	
			ROGUELIKE::draw_player_projectile(self, &bullet, &player);	

			ROGUELIKE::draw_weapon(self, &player,&sword);
			
			// UPDATE OBSTACLES
			// function to check explosive barrels stuff like that should go here. placed for ordering.

			// CHECK COLLISIONS
			ROGUELIKE::check_collisions(self, &mut player, &mut enemies);
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
	pub fn create_rooms(mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut rng = rand::thread_rng();

		let mut new_map = map;

		let num_attempts = 200;
		let mut count = 0;
		while count < num_attempts {
			count += 1;
			let x = rng.gen_range(0..MAP_SIZE_W);
			let y = rng.gen_range(0..MAP_SIZE_H);
			let width = rng.gen_range(11..21);
			let height = rng.gen_range(11..21);
			if x % 2 == 0 || y % 2 == 0 || width % 2 == 0 || height % 2 == 0 {
				count -= 1;
				continue;
			}
			if x + width < MAP_SIZE_W && y + height < MAP_SIZE_H {
				let mut collided = false;
				for j in 0..width {
					for k in 0..height {
						if x > 1 && y > 1 {
							if new_map[x - 1][y - 1] == 1 {
								collided = true;
							}
						}
						if x > 1 {
							if new_map[x - 1][y + k] == 1 {
								collided = true;
							}
						}
						if y > 1 {
							if new_map[x + j][y - 1] == 1 {
								collided = true;
							}
						}
						if new_map[x + j + 1][y + k + 1] == 1 {
							collided = true;
						}
					}
				}
				if collided {
					continue;
				}
				for j in 0..width {
					for k in 0..height {
						new_map[x + j][y + k] = 1;
					}
				}
				count += 1;
			}
		}
		
		return new_map;
	}

	pub fn build_maze(mut x: usize, mut y: usize, mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut new_map = map;

		let mut count = 0;
		let target = 300;
		let mut direction = 0;
		while count < target {
			count += 1;
			new_map[x][y] = 7;
			if count > 1 {
				match direction {
					0 => new_map[x][y + 1] = 7,
					1 => new_map[x - 1][y] = 7,
					2 => new_map[x][y - 1] = 7,
					3 => new_map[x + 1][y] = 7,
					_ => new_map[x][y] = 7,
				}
			}

			println!("");
			for i in 0..MAP_SIZE_W {
				for j in 0..MAP_SIZE_H {
					print!("{} ", new_map[j][i]);
				}
				println!("");
			}

			// North
			if y > 2 {
				if new_map[x][y - 2] == 0 {
					y = y - 2;
					direction = 0;
					println!("West");
					continue;
				}
			}
			// East
			if x < MAP_SIZE_W - 2 {
				if new_map[x + 2][y] == 0 {
					x = x + 2;
					direction = 1;
					println!("South");
					continue;
				}
			}
			// South
			if y < MAP_SIZE_H - 2 {
				if new_map[x][y + 2] == 0 {
					y = y + 2;
					direction = 2;
					println!("East");
					continue;
				}
			}
			// West
			if x > 2 {
				if new_map[x - 2][y] == 0 {
					x = x - 2;
					direction = 3;
					println!("North");
					continue;
				}
			}
		}

		return new_map;
	}

	pub fn create_maze(mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let x = 1;
		let y = 1;
		let mut new_map = ROGUELIKE::build_maze(x, y, map);

		return new_map;
	}

	pub fn create_walls(mut map: [[i32; MAP_SIZE_W]; MAP_SIZE_H]) -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut new_map = map;

		for i in 0..MAP_SIZE_W as i32 {
			for j in 0..MAP_SIZE_H as i32 {
				if new_map[i as usize][j as usize] == 0 {
					for k in 0..3 as i32 {
						for l in 0..3 as i32 {
							if i + k - 1 < 0 ||
							   j + l - 1 < 0 ||
							   i + k - 1 >= MAP_SIZE_W as i32 ||
							   j + l - 1 >= MAP_SIZE_H as i32 {
								continue;
							}
							if new_map[i as usize + k as usize - 1][j as usize + l as usize - 1] == 1 {
								new_map[i as usize][j as usize] = 2;
							}
						}
					}
				}
			}
		}

		return new_map;
	}

	pub fn create_map() -> [[i32; MAP_SIZE_W]; MAP_SIZE_H] {
		let mut map = [[0; MAP_SIZE_W]; MAP_SIZE_H];

		map = ROGUELIKE::create_rooms(map);
		map = ROGUELIKE::create_maze(map);
		// map = ROGUELIKE::create_walls(map);

		return map;
	}

	pub fn update_background(&mut self, player: &Player, background: &mut Background) -> Result<(), String> {
		background.set_curr_background(player.x(), player.y(), player.width(), player.height());
		let tiles = &self.game_data.rooms[self.game_data.current_room].tiles;
		let mut n = 0;
		for i in 0..self.game_data.rooms[0].xwalls.1+1 {
			for j in 0..self.game_data.rooms[0].ywalls.1+1 {
				if tiles[n].0 {
					let t = background.get_tile_info(tiles[n].1, i, j, player.x(), player.y());
					self.core.wincan.copy(t.0, t.1, t.2)?;
				}
				n+=1;
			}
		}
		Ok(())
	}
	// update enemies
	pub fn update_enemies(&mut self, rngt: &mut Vec<i32>, enemies: &mut Vec<Enemy>, player: &Player) -> Vec<i32> {
		let mut i = 1;
		for enemy in enemies {
			if enemy.is_alive(){
				enemy.check_attack(&mut self.game_data, (player.x(), player.y()));
				if rngt[0] > 30 || enemy.force_move(&self.game_data){
					rngt[i] = rand::thread_rng().gen_range(1..5);
					rngt[0] = 0;
				}
				let t = enemy.update_pos(&self.game_data, rngt, i, (player.x(), player.y()));
				self.core.wincan.copy(enemy.txtre(), enemy.src(), t).unwrap();
				i += 1;
			}
		}
		rngt[0] += 1;
		return rngt.to_vec();
	}

	pub fn update_gold(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, coin_texture: &Texture) -> Result<(), String> {
		//add coins to gold vector
		for enemy in enemies {
			if !enemy.is_alive() && enemy.has_gold(){	// Should be changed to has_drop() when more drops
				let drop = enemy.drop_item();
				self.game_data.gold.push(drop);
			}
		}
		for coin in self.game_data.gold.iter_mut() {
			if !coin.collected() {
				let pos = Rect::new(coin.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									coin.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);

				self.core.wincan.copy(&coin_texture, coin.src(), pos)?;
			}
		}
		Ok(())
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
			//println!("{} {}", player.x(), player.y());
			
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
	fn check_collisions(&mut self, player: &mut Player, enemies: &mut Vec<Enemy>) {
		let xbounds = self.game_data.rooms[0].xbounds;
		let ybounds = self.game_data.rooms[0].ybounds;
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
		for coin in self.game_data.gold.iter_mut() {
			if check_collision(&player.pos(), &coin.pos()) {
				if !coin.collected() {
					coin.set_collected();
					player.add_coins(coin.get_gold());
				}
			}
		}
	}

	//draw weapon
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

	// draw player
	pub fn draw_player(&mut self, count: &i32, f_display: &i32, player: &mut Player, curr_bg: Rect) {
		player.set_cam_pos(curr_bg.x(), curr_bg.y());
		player.get_frame_display(count, f_display);
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
	}

	pub fn draw_player_projectile(&mut self, bullet: &Texture, player: &Player)-> Result<(), String>  {
		let texture_creator = self.core.wincan.texture_creator();
		for projectile in self.game_data.player_projectiles.iter_mut() {
			let mut p = texture_creator.load_texture("images/abilities/bullet.png")?;

			if projectile.is_active(){
				match projectile.p_type{
					ProjectileType::Bullet=>{
						 p = texture_creator.load_texture("images/abilities/bullet.png")?;
					}
					ProjectileType::Fireball=>{
						 p = texture_creator.load_texture("images/abilities/beng.png")?;
					}
				}	
				self.core.wincan.copy(&p, projectile.src(), projectile.offset_pos(player));
			}
		}
		Ok(())
	}
	pub fn draw_enemy_projectile(&mut self, bullet: &Texture, player: &Player) {
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active(){
				self.core.wincan.copy(&bullet, projectile.src(), projectile.offset_pos(player));
			}
		}
	}
}
