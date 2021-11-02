extern crate rogue_sdl;
mod enemy;
mod background;
mod player;
mod ui;
mod projectile;
mod credits;
mod gamedata;
mod gold;
mod room;

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use rogue_sdl::{Game, SDLCore};
use crate::gamedata::GameData;
use crate::enemy::*;
use crate::projectile::*;
use crate::player::*;
use crate::background::*;

// window globals
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE + (TILE_SIZE / 2);

const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

//background globals
//const BG_W: u32 = 2400;
//const BG_H: u32 = 1440;

// game globals
const SPEED_LIMIT: f64 = 200.0;
const ACCEL_RATE: f64 = 200.0;
//const STARTING_TIMER: u128 = 1000;

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
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

// CREATE GAME
impl Game for ROGUELIKE {

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

		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		let fire_texture = texture_creator.load_texture("images/abilities/fireball.png")?;
		let bullet = texture_creator.load_texture("images/abilities/bullet.png")?;
		let coin_texture = texture_creator.load_texture("images/ui/gold_coin.png")?;

		let mut enemies: Vec<Enemy> = Vec::with_capacity(5);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement. if we make an entities file, this should probably be moved there
		let mut i=1;
		for _ in 0..enemies.capacity(){
			let num = rng.gen_range(1..5);
			let enemy_type: String; 
			match num {
				5 => { enemy_type = String::from("ranged") } 
				4 => { enemy_type = String::from("ranged") } 
				_ => { enemy_type = String::from("melee") } 
			}
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
			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}
		// SETUP ARRAY OF PROJECTILES
		// let mut projectiles: Vec<Projectile> = Vec::with_capacity(3);

		/* // CREATE FIREBALL (SHOULD BE MOVED TO fireball.rs WHEN CREATED)
        let mut fireball = ranged_attack::RangedAttack::new(
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			false,
			false,
			0,
            texture_creator.load_texture("images/abilities/fireball.png")?,
		); */

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
			ROGUELIKE::update_interactables(self, &mut enemies, &mut player, &coin_texture);

			// UPDATE ATTACKS
			// Should be switched to take in array of active fireballs, bullets, etc.
			ROGUELIKE::update_projectiles(&mut self.game_data.player_projectiles, &mut self.game_data.enemy_projectiles);
			ROGUELIKE::draw_projectile(self, &bullet, &player, 0.0)?;	
			ROGUELIKE::draw_weapon(self, &mut player)?;
			
			// UPDATE OBSTACLES
			// function to check explosive barrels stuff like that should go here. placed for ordering. 			

			// CHECK COLLISIONS
			ROGUELIKE::check_collisions(self, &mut player, &mut enemies);
			if player.is_dead(){
				break 'gameloop;
			}

			// UPDATE UI
			ROGUELIKE::update_ui(self, &player)?;
			
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

// Create map
impl ROGUELIKE {
	pub fn create_map() -> [[i32; 10]; 10] {
		let mut map = [[0; 10]; 10];
		for i in 0..10 {
			for j in 0..10 {
				println!("{}", map[i][j]);
			}
		}
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

	pub fn update_interactables(&mut self, enemies: &mut Vec<Enemy>, player: &mut Player, coin_texture: &Texture) -> Result<(), String> {
		//add coins to gold vector
		for enemy in enemies {
			if !enemy.is_alive() {
				if enemy.has_gold() {
					let coin = gold::Gold::new(
						Rect::new(
							enemy.x() as i32,
							enemy.y() as i32,
							TILE_SIZE,
							TILE_SIZE,
						),

					);
					self.game_data.gold.push(coin);
					enemy.set_no_gold();
				}
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
	pub fn check_inputs(&mut self, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player) {
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
				let bullet = player.fire(mousestate.x(), mousestate.y(), self.game_data.get_speed_limit());
				self.game_data.player_projectiles.push(bullet);
			}
		}
		//ability
		if keystate.contains(&Keycode::F){
			println!("you found the easter egg");
		}
		// FOR TESTING ONLY: USE TO FOR PRINT VALUES
		if keystate.contains(&Keycode::P) {
			//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
			//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
			//println!("{} {}", player.x(), player.y());
		}
			
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
		player.set_invincible();
	}

	//draw weapon
	pub fn draw_weapon(&mut self, player: &mut Player) -> Result<(), String> {

		let texture_creator = self.core.wincan.texture_creator();
		let sword = texture_creator.load_texture("images/player/sword_l.png")?;
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

		self.core.wincan.copy_ex(&sword, None, pos, angle, rotation_point, player.facing_right, false).unwrap();
		Ok(())
	}

	//update background
	pub fn update_ui(&mut self, player: &Player) -> Result<(), String> {
		// set ui bar
		let texture_creator = self.core.wincan.texture_creator();
		let src = Rect::new(0, 0, CAM_W, TILE_SIZE*2);
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 16, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide_yellow.png")?;
		self.core.wincan.copy(&ui, src, pos)?;
		let pos = Rect::new(0, (CAM_H - TILE_SIZE) as i32 - 8, CAM_W, TILE_SIZE*3/2);
		let ui = texture_creator.load_texture("images/ui/bb_wide.png")?;
		self.core.wincan.copy(&ui, src, pos)?;
		let ttf_creator = sdl2::ttf::init().map_err( |e| e.to_string() )?;
		let get_font = ttf_creator.load_font("font/comic_sans.ttf", 80)?;

		//create hearts
		let mut i=0;
		while i+10 < player.get_hp() {
			let heart = ui::UI::new(
				Rect::new(
					(i/10) as i32 *(TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				), 
				texture_creator.load_texture("images/ui/heart.png")?,
			);
			self.core.wincan.copy(heart.texture(), heart.src(), heart.pos())?;
			i+=10;
		}
		
		let mut texture = texture_creator.load_texture("images/ui/heart.png")? ;
		if  player.get_hp()%10 != 0  {
			texture = texture_creator.load_texture("images/ui/half_heart.png")?;
		}
			let half_heart = ui::UI::new(
				Rect::new(
					(i/10) as i32 * (TILE_SIZE as f64 *1.2) as i32,
					(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
					(TILE_SIZE as f64 *1.2) as u32,
					(TILE_SIZE as f64 *1.2) as u32,
				),
				texture,
			);
		self.core.wincan.copy(half_heart.texture(), half_heart.src(), half_heart.pos())?;

		//display mana
		let mut mana = ui::UI::new(
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
		self.core.wincan.copy(mana.texture(), mana.src(), mana.pos())?;

		//get current mana as a string
		let mana = player.get_mana();
		let max_mana = player.get_max_mana();
		let mut s: String = mana.to_string();
		let a: String = max_mana.to_string();
		s += "/";
		s += &a;

		//display string next to mana

		//display equipped waepon
		if player.get_curr_meele() == "sword_l"
		{
			let weapon = ui::UI::new(
				Rect::new(
					(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*8) as i32,
					(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
					(TILE_SIZE as f64 * 1.2) as u32,
					(TILE_SIZE as f64 * 1.2) as u32,
				),
				texture_creator.load_texture("images/player/sword_l.png")?,
			);
			self.core.wincan.copy(weapon.texture(), weapon.src(),weapon.pos())?;
		}


		if player.get_curr_ability() == "bullet"
		{
			let ability = ui::UI::new(
				Rect::new(
					(CAM_W-((TILE_SIZE as f64 * 1.2) as u32)*6) as i32,
					(CAM_H-(TILE_SIZE as f64 * 1.2) as u32) as i32,
					(TILE_SIZE as f64 * 1.2) as u32,
					(TILE_SIZE as f64 * 1.2) as u32,
				),
				texture_creator.load_texture("images/abilities/bullet.png")?,
			);
			self.core.wincan.copy(ability.texture(), ability.src(),ability.pos())?;
		}
		// create coins
		let coin = ui::UI::new(
			Rect::new(
				(CAM_W-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(CAM_H-(TILE_SIZE as f64 *1.2) as u32) as i32,
				(TILE_SIZE as f64 *1.2) as u32,
				(TILE_SIZE as f64 *1.2) as u32,
			),
			texture_creator.load_texture("images/ui/gold_coin.png")?,
		);
		self.core.wincan.copy(coin.texture(), coin.src(), coin.pos())?;
		let coin_count = get_font.render( format!("{}", player.get_coins() ).as_str() ).blended(Color::WHITE).unwrap();
		let display_coin_count = texture_creator.create_texture_from_surface( &coin_count ).unwrap();
		self.core.wincan.copy(&display_coin_count, None, Rect::new( coin.pos().x - 16 as i32, coin.pos().y + 12 as i32, 32, 48) );
																//(text to display, src(none), (positionx, positiony, sizex, sizey))
		Ok(())
	}

	// draw player
	pub fn draw_player(&mut self, count: &i32, f_display: &i32, player: &mut Player, curr_bg: Rect) {
		player.set_cam_pos(curr_bg.x(), curr_bg.y());
		player.get_frame_display(count, f_display);
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
	}

	pub fn draw_projectile(&mut self, bullet: &Texture, player: &Player, angle: f64) -> Result<(), String> {
		for projectile in self.game_data.player_projectiles.iter_mut() {
			if projectile.is_active(){
				let pos = Rect::new(projectile.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									projectile.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy(&bullet, projectile.src(), pos)?; // rotation center
			}
		}
		let p = Point::new(0, (TILE_SIZE/2) as i32);
		for projectile in self.game_data.enemy_projectiles.iter_mut() {
			if projectile.is_active(){
				let pos = Rect::new(projectile.x() as i32 + (CENTER_W - player.x() as i32), //screen coordinates
									projectile.y() as i32 + (CENTER_H - player.y() as i32),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy_ex(&bullet, None, pos, angle, p, false, false)?; // rotation center
			}
		}
		Ok(())
	}
}
