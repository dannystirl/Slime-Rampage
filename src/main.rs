extern crate rogue_sdl;
mod enemy;
mod background;
mod player;
mod ranged_attack;
mod ui;
mod credits;

use std::collections::HashSet;
//use std::time::Duration;
//use std::time::Instant;
use rand::Rng;
use crate::enemy::*;
use crate::ranged_attack::*;
use crate::player::*;
use crate::background::*;
use crate::ui::*;

use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
//use sdl2::mouse::MouseButtonIterator;
//use sdl2::mouse::PressedMouseButtonIterator;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
//use sdl2::render::WindowCanvas;
//use sdl2::render::Texture;

use rogue_sdl::{Game, SDLCore};

// window globals
const TITLE: &str = "Roguelike";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TILE_SIZE: u32 = 64;
const ATTACK_LENGTH: u32 = TILE_SIZE + (TILE_SIZE / 2);

const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE / 2) as i32;
const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE / 2) as i32;

//background globals
const BG_W: u32 = 2400;
const BG_H: u32 = 1440;

// game globals
const SPEED_LIMIT: i32 = 3;
const ACCEL_RATE: i32 = 3;

pub struct ROGUELIKE {
	core: SDLCore,
}

// calculate velocity resistance
fn resist(vel: i32, delta: i32) -> i32 {
	if delta == 0 {
		if vel > 0 {-1}
		else if vel < 0 {1}
		else {delta}
	} else {delta}
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
		Ok(ROGUELIKE{ core })
	}

	fn run(&mut self) -> Result<(), String> {
        let texture_creator = self.core.wincan.texture_creator();
		let mut rng = rand::thread_rng();

		let mut count = 0;
		let f_display = 15;

		// CREATE PLAYER SHOULD BE MOVED TO player.rs
		let mut player = player::Player::new(
			(CENTER_W, CENTER_H,),
			texture_creator.load_texture("images/player/slime_sheet.png")?,
		);

		// INITIALIZE ARRAY OF ENEMIES (SHOULD BE MOVED TO room.rs WHEN CREATED)
		let mut enemies: Vec<Enemy> = Vec::with_capacity(2);	// Size is max number of enemies
		let mut rngt = vec![0; enemies.capacity()+1]; // rngt[0] is the timer for the enemys choice of movement
		let mut i=1;
		for _ in 0..enemies.capacity(){
			let e = enemy::Enemy::new(
				Rect::new(
					(CAM_W/2 - TILE_SIZE/2 + 200) as i32,
					(CAM_H/2 - TILE_SIZE/2) as i32,
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/enemies/place_holder_enemy.png")?,
			);
			enemies.push(e);
			rngt[i] = rng.gen_range(1..5); // decides if an enemy moves
			i+=1;
		}

		// CREATE FIREBALL (SHOULD BE MOVED TO fireball.rs WHEN CREATED)
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
		);

		// CREATE ROOM 
		let xwalls: (i32, i32) = (1,rng.gen_range(19..27));
		let ywalls: (i32, i32) = (1,rng.gen_range(10..19));
		let xbounds: (i32,i32) = ((xwalls.0*TILE_SIZE as i32), ( (xwalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
		let ybounds: (i32,i32) = ((ywalls.0*TILE_SIZE as i32), ( (ywalls.1 as u32 *TILE_SIZE)-TILE_SIZE) as i32);
		
		let mut background = background::Background::new(
			texture_creator.load_texture("images/background/bb.png")?,
			texture_creator.load_texture("images/background/floor_tile_1.png")?, 
			texture_creator.load_texture("images/background/floor_tile_2.png")?, 
			texture_creator.load_texture("images/background/floor_tile_maroon.png")?, 
			texture_creator.load_texture("images/background/floor_tile_pilar.png")?, 
			xwalls, 
			ywalls, 
		);

		// obstacles that everything should collide with
		#[allow(unused_variables)]
		let obstacle_pos = background.create_new_map(xwalls, ywalls);

		// MAIN GAME LOOP
		'gameloop: loop {
			// reset frame
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			player.set_x_delta(0);
			player.set_y_delta(0);

			let mousestate= self.core.event_pump.mouse_state();
			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

				// FOR TESTING ONLY: USE TO FOR PRINT VALUES
				if keystate.contains(&Keycode::P) {
					//println!("\nx:{} y:{} ", enemies[0].x() as i32, enemies[0].y() as i32);
					//println!("{} {} {} {}", enemies[0].x() as i32, enemies[0].x() as i32 + (enemies[0].width() as i32), enemies[0].y() as i32, enemies[0].y() as i32 + (enemies[0].height() as i32));
					//println!("{} {}", player.x(), player.y());
				}
			// CLEAR BACKGROUND
            self.core.wincan.copy(&background.black, None, None)?;

			// UPDATE BACKGROUND
			let cur_bg = Rect::new(
				(player.x() + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32),
				(player.y() + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32),
				CAM_W,
				CAM_H,
			);
			ROGUELIKE::update_background(self, xwalls, ywalls, &player, &background)?;

			// UPDATE ENEMIES
			rngt = ROGUELIKE::update_enemies(self, xwalls, ywalls, xbounds, ybounds, rngt, &mut enemies, &mut player);

			// UPDATE PLAYER
			ROGUELIKE::check_inputs(&mut fireball, &keystate, mousestate, &mut player);
			ROGUELIKE::update_player(xwalls, ywalls, xbounds, ybounds, &mut player, &obstacle_pos);
			self.draw_player(xwalls, ywalls, &count, &f_display, &mut player, &cur_bg);
			count = count + 1;
			if count > f_display * 5 {
				count = 0;
			}

			// UPDATE ATTACKS
			// Should be switched to take in array of active fireballs, bullets, etc.
			self.update_projectiles(&mut fireball);
			ROGUELIKE::display_weapon(self, &mut player)?;
			
			// UPDATE OBSTACLES
			// function to check explosive barrels stuff like that should go here. placed for ordering. 			

			// CHECK COLLISIONS
			ROGUELIKE::check_collisions(xbounds, ybounds, &mut player, &mut enemies);
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
	pub fn update_background(&mut self, xwalls: (i32,i32), ywalls: (i32,i32), player: &Player, background:& Background) -> Result<(), String> {
		let cam_delta = ROGUELIKE::stop_camera(xwalls, ywalls, &player);
		let mut n = 0;
		for i in 0..xwalls.1+1 {
			for j in 0..ywalls.1+1 {
				if background.tiles[n].0 {
					let num = background.tiles[n].1;
					let texture;
					match num {
						7 => { texture = &background.texture_3 } // pillar 
						6 => { texture = &background.texture_2 } // border tiles
						1 => { texture = &background.texture_1 } // leaves on tile
						_ => { texture = &background.texture_0 } // regular tile
					}
					// double tile size 
					let src;
					let pos;
					if num==7 {
						src = Rect::new(0, 0, TILE_SIZE*2, TILE_SIZE*2);
						pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - player.x()- cam_delta.0),
											j * TILE_SIZE as i32 + (CENTER_H - player.y() - cam_delta.1),
											TILE_SIZE*2, TILE_SIZE*2);
					} else {
						src = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
						pos = Rect::new(i * TILE_SIZE as i32 + (CENTER_W - player.x()-cam_delta.0),
											j * TILE_SIZE as i32 + (CENTER_H - player.y() - cam_delta.1),
											TILE_SIZE, TILE_SIZE);
					}
					self.core.wincan.copy(texture, src, pos)?;
				}
				n+=1;
			}
		}
		Ok(())
	}
	// update enemies
	pub fn update_enemies(&mut self, xwalls: (i32,i32), ywalls: (i32,i32), xbounds: (i32,i32), ybounds: (i32,i32), mut rngt: Vec<i32>, enemies: &mut Vec<Enemy>, player: &mut Player) -> Vec<i32>{
		let mut i = 1;
		let mut rng = rand::thread_rng();
		for enemy in enemies {
			if enemy.is_alive() {
				let cam_delta = ROGUELIKE::stop_camera(xwalls, ywalls, &player);
				if rngt[0] > 30 || ROGUELIKE::check_edge(xbounds, ybounds, &enemy){
					rngt[i] = rng.gen_range(1..5);
					rngt[0] = 0;
				}
				let x_d = (enemy.x() as i32 - player.x()).pow(2);
				let y_d = (enemy.y() as i32 - player.y()).pow(2);
				let distance = ((x_d + y_d) as f64).sqrt();
				if enemy.get_stun_timer() > 1000 {
					enemy.set_stunned(false);
				} else {
					enemy.slow_vel(0.1);
					let angle = enemy.angle();
					let mut x = (enemy.get_vel() * -1.0) * angle.sin();
					if enemy.x_flipped {
						x *= -1.0;
					}
					let mut y = (enemy.get_vel() * -1.0) * angle.cos();
					if enemy.y_flipped {
						y *= -1.0;
					}
					enemy.pos.set_x(((enemy.x() + x) as i32).clamp(xbounds.0, xbounds.1));
					enemy.pos.set_y(((enemy.y() + y) as i32).clamp(ybounds.0, ybounds.1));
				}
				if distance > 300.0 {
					enemy.update_pos(rngt[i], xbounds, ybounds);
				} else {
					enemy.aggro(player.x().into(), player.y().into(), xbounds, ybounds);
				}
				let pos = Rect::new(enemy.x() as i32 + (CENTER_W - player.x() - cam_delta.0),
									enemy.y() as i32 + (CENTER_H - player.y() - cam_delta.1),
									TILE_SIZE, TILE_SIZE);
				self.core.wincan.copy(enemy.txtre(), enemy.src(), pos).unwrap();
				i += 1;
			}
		}
		rngt[0] += 1;
		return rngt;
	}

	// check input values
	pub fn check_inputs(fireball: &mut RangedAttack, keystate: &HashSet<Keycode>, mousestate: MouseState, mut player: &mut Player) {
		// move up
		if keystate.contains(&Keycode::W) {
			player.set_y_delta(player.y_delta() - ACCEL_RATE);
			player.is_still = false;
		}
		// move left
		if keystate.contains(&Keycode::A) {
			player.set_x_delta(player.x_delta() - ACCEL_RATE);
			player.facing_right = false;
			player.is_still = false;
		}
		// move down
		if keystate.contains(&Keycode::S) {
			player.set_y_delta(player.y_delta() + ACCEL_RATE);
			player.is_still = false;
		}
		// move right
		if keystate.contains(&Keycode::D) {
			player.set_x_delta(player.x_delta() + ACCEL_RATE);
			player.facing_right = true;
			player.is_still = false;
		}
		// basic attack
		if mousestate.left() || keystate.contains(&Keycode::Space) {
			if !(player.is_attacking) {
				/*println!(
					"X = {:?}, Y = {:?}",
					mousestate.x(),
					mousestate.y(),
				);*/
				// player.base_attack(mousestate.x(), mousestate.y());
				player.attack();
			}
		}
		// shoot fireball
		if keystate.contains(&Keycode::F) && fireball.frame() == 0 {
			fireball.set_use(true);
			fireball.start_pos(player.get_cam_pos().x(), player.get_cam_pos().y(), player.facing_right);
		}
	}

	// update projectiles
	pub fn update_projectiles(&mut self, fireball: &mut RangedAttack) {
		if fireball.in_use() {
			fireball.set_frame(fireball.frame() + 1);
			fireball.update_pos((0, (CAM_W - TILE_SIZE) as i32));
			if fireball.frame() == 28 {
				fireball.set_use(false);
				fireball.set_frame(0);
			}
			// this needs to be mirrored
			self.core.wincan.copy_ex(fireball.texture(), fireball.src(4, 7), fireball.pos(), 0.0, None, fireball.facing_right, false).unwrap();
		}
	}

	// check collisions
	fn check_collisions(xbounds: (i32,i32), ybounds: (i32,i32), player: &mut Player, enemies: &mut Vec<Enemy>) {
		for enemy in enemies {
			if check_collision(&player.pos(), &enemy.pos()) {
				player.minus_hp(5.0);
				//println!("Health: {}", player.get_hp()); //for debugging
			}

			if player.is_attacking {
				if check_collision(&player.get_attack_box(), &enemy.pos()) {
					enemy.knockback(player.x().into(), player.y().into(), xbounds, ybounds);
					enemy.minus_hp(1.0);
				}
			}
		}
		player.set_invincible();
	}

	// update player
	fn update_player(xwalls: (i32,i32), ywalls: (i32,i32), xbounds: (i32,i32), ybounds: (i32,i32), mut player: &mut Player, obstacle_pos: &Vec<(i32,i32)> ) {
		// Slow down to 0 vel if no input and non-zero velocity
		player.set_x_delta(resist(player.x_vel(), player.x_delta()));
		player.set_y_delta(resist(player.y_vel(), player.y_delta()));

		// set animation when player is not moving
		if player.x_vel() == 0 && player.y_vel() == 0 { player.is_still = true; }

		// Don't exceed speed limit
		player.set_x_vel((player.x_vel() + player.x_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));
		player.set_y_vel((player.y_vel() + player.y_delta()).clamp(-SPEED_LIMIT, SPEED_LIMIT));

		// Stay inside the viewing window
		player.set_x((player.x() + player.x_vel()).clamp(0, xwalls.1 * TILE_SIZE as i32));
		player.set_y((player.y() + player.y_vel()).clamp(0, ywalls.1 * TILE_SIZE as i32));

		for ob in obstacle_pos {
			let obs = Rect::new(ob.0 * TILE_SIZE as i32, ob.1 * TILE_SIZE as i32, TILE_SIZE*2, TILE_SIZE*2);
			if check_collision(&player.pos(), &obs) {
				// collision on object top
				if (player.pos().bottom() >= obs.top()) && (player.pos().bottom() < obs.bottom()) 		// check y bounds
				&& (player.pos().left() > obs.left()) && (player.pos().right() < obs.right()) {			// prevent x moves
					player.set_y((player.y() + player.y_vel()).clamp(0, (ob.1-1) * TILE_SIZE as i32));
				// collision on object bottom
				} else if (player.pos().top() < obs.bottom()) && (player.pos().top() > obs.top()) 		// check y bounds
				&& (player.pos().left() > obs.left()) && (player.pos().right() < obs.right()) {			// prevent x moves
					player.set_y((player.y() + player.y_vel()).clamp((ob.1+2)*TILE_SIZE as i32, ywalls.1 * TILE_SIZE as i32));
				// collision on object left 
				} else if (player.pos().right() > obs.left()) && (player.pos().right() < obs.right())	// check x bounds
					   && (player.pos().top() > obs.top()) && (player.pos().bottom() < obs.bottom()) {	// prevent y moves
					player.set_x((player.x() + player.x_vel()).clamp(0, (ob.0-1) * TILE_SIZE as i32));
					// collision on object right
				} else if (player.pos().left() < obs.right()) && (player.pos().left() > obs.left()) 	// check x bounds
					   && (player.pos().top() > obs.top()) && (player.pos().bottom() < obs.bottom()) {	// prevent y moves
					player.set_x((player.x() + player.x_vel()).clamp((ob.0+2)*TILE_SIZE as i32, xwalls.1 * TILE_SIZE as i32));
				}
			}
		}

		player.update_pos(xbounds, ybounds);

		if player.is_attacking { player.set_attack_box(player.x(), player.y()); }

		if player.get_attack_timer() > player.get_cooldown() {
			player.set_cooldown();
		}
	}

	//update background
	pub fn unused_background(&mut self, player: &mut Player, background: &mut Background) -> Result<(), String> {
		let cur_bg = Rect::new(
			((player.x() + ((player.width() / 2) as i32)) - ((CAM_W / 2) as i32)).clamp(0, (BG_W - CAM_W) as i32),
			((player.y() + ((player.height() / 2) as i32)) - ((CAM_H / 2) as i32)).clamp(0, (BG_H - CAM_H) as i32),
			CAM_W,
			CAM_H,
		);
		self.core.wincan.set_draw_color(Color::BLACK);
		self.core.wincan.clear();

		// Draw subset of bg
		self.core.wincan.copy(background.texture(), cur_bg, None).unwrap();
		Ok(())
	}

	//draw weapon
	pub fn display_weapon(&mut self, player: &mut Player) -> Result<(), String> {
		if player.is_attacking {
			let texture_creator = self.core.wincan.texture_creator();
			let sword = texture_creator.load_texture("images/player/sword_l.png")?;

			let mut src = Rect::new(player.get_cam_pos().x() - ATTACK_LENGTH as i32, player.get_cam_pos().y(), ATTACK_LENGTH, TILE_SIZE);
			if player.facing_right {
				src = Rect::new(player.get_cam_pos().x() + TILE_SIZE as i32, player.get_cam_pos().y(), ATTACK_LENGTH, TILE_SIZE);
			}
			if player.weapon_frame > 30 { player.weapon_frame=0}
			player.weapon_frame+=1;

			//naive weapon animation 
			let angle = -30.0;
			let p;
			if player.facing_right{
				p = Point::new(0, (TILE_SIZE/2) as i32);//rotation center
			} else{
				p = Point::new(ATTACK_LENGTH as i32,  (TILE_SIZE/2)  as i32);//rotation center
			}

			if player.weapon_frame < 15{
				self.core.wincan.copy_ex(&sword, None, src, angle, p, player.facing_right, false).unwrap();
				
			}else{
				self.core.wincan.copy_ex(&sword, None, src, -angle, p, player.facing_right, false).unwrap();
			}
		}
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

		//create 3 hearts
		let mut i=0;
		while i < player.get_hp() as i32 /3 {
			let heart = ui::UI::new(
				Rect::new(
					i*TILE_SIZE as i32,
					(CAM_H-TILE_SIZE) as i32,
					TILE_SIZE,
					TILE_SIZE,
				),
				texture_creator.load_texture("images/ui/heart.png")?,
			);
			self.core.wincan.copy(heart.texture(), heart.src(), heart.pos())?;
			i+=1;
		}
		Ok(())
	}

	// draw player
	pub fn draw_player(&mut self, xwalls: (i32,i32), ywalls: (i32,i32), count: &i32, f_display: &i32, player: &mut Player, cur_bg: &Rect) {
		let cam_delta = ROGUELIKE::stop_camera(xwalls, ywalls, &player);
		player.set_cam_pos(cur_bg.x()+cam_delta.0, cur_bg.y()+cam_delta.1);

		// I think it looks better when doing animation constantly, we can keep 
		// the if statement if we decide to add a specific moving animation

		//if !player.is_still {
			player.get_frame_display(count, f_display);
		/*} else {
			player.set_src(0, 0);
		}*/
		self.core.wincan.copy_ex(player.texture_all(), player.src(), player.get_cam_pos(), 0.0, None, player.facing_right, false).unwrap();
	}

	pub fn stop_camera(xwalls: (i32,i32), ywalls: (i32,i32), player: &Player) -> (i32,i32) {
		let mut cam_delta_y = 0;
		if player.y() > (ywalls.1-4)*(TILE_SIZE as i32) {
			cam_delta_y = ((ywalls.1-4) * TILE_SIZE as i32)+(TILE_SIZE as i32)/8 - player.y();
		} else if player.y() < ((5*TILE_SIZE) as i32)+(TILE_SIZE as i32)/8{
			cam_delta_y = (5*(TILE_SIZE as i32)+(TILE_SIZE as i32)/8) - player.y();
		}
		let mut cam_delta_x = 0;
		if player.x() > (xwalls.1-9)*(TILE_SIZE as i32)-(TILE_SIZE as i32)/2 {
			cam_delta_x = (xwalls.1-9)*(TILE_SIZE as i32)-(TILE_SIZE as i32)/2 - player.x();
		} else if player.x() < (9*TILE_SIZE) as i32 + (TILE_SIZE as i32)/2{
			cam_delta_x = 9*(TILE_SIZE as i32) + (TILE_SIZE as i32)/2 - player.x();
		}
		return (cam_delta_x, cam_delta_y);
	}

	// force enemy movement
	pub fn check_edge(xbounds: (i32,i32), ybounds: (i32,i32), enemy: &enemy::Enemy) -> bool{
		if  enemy.x() <= xbounds.0 as f64 ||
		enemy.x() >=  xbounds.1 as f64 ||
		enemy.y() <= ybounds.0 as f64||
		enemy.y() >= ybounds.1 as f64
		{return true;}
		else {return false;}
	}
}
