pub const DEVELOP: bool = false; 
pub const DEBUG: bool = true; 

use rand::Rng;
use sdl2::rect::Rect;
use std::time::Instant;

use crate::gold::*;
use crate::projectile::*;
use crate::room::*;
//use crate::map::*;
use crate::crateobj::*;

// window globals
pub const TITLE: &str = "Roguelike";
pub const CAM_W: u32 = 1280;
pub const CAM_H: u32 = 720;
pub const TILE_SIZE_64: u32 = 64;                       // tile sizes are all 64 px
pub const TILE_SIZE: u32 = 32;                          // sprite sizes vary 
pub const TILE_SIZE_HALF: u32 = TILE_SIZE/2;            // generic half tile size
pub const TILE_SIZE_CAM: u32 = TILE_SIZE;               // cam position 
pub const TILE_SIZE_PLAYER: u32 = TILE_SIZE_CAM*4/5;    // hitboxes are slightly smaller than visual hitboxes

pub const CENTER_W: i32 = (CAM_W / 2 - TILE_SIZE_HALF) as i32;
pub const CENTER_H: i32 = (CAM_H / 2 - TILE_SIZE_HALF) as i32;

// room globals
pub const MIN_ROOM_W: usize = 11;
pub const MAX_ROOM_W: usize = 21;
pub const MAP_SIZE_W: usize = 51;
pub const MIN_ROOM_H: usize = 11;
pub const MAX_ROOM_H: usize = 21;
pub const MAP_SIZE_H: usize = 51;

// game globals
pub const SPEED_LIMIT: f64 = 200.0;
pub const ACCEL_RATE: f64 = 100.0;
//pub const STARTING_TIMER: u128 = 1000;

// player globals
pub const ATTACK_LENGTH: u32 = TILE_SIZE * 3 / 2;
pub const ATTK_COOLDOWN: u128 = 300;
pub const DMG_COOLDOWN: u128 = 800;
pub const FIRE_COOLDOWN_P: u128 = 300;
pub const MANA_RESTORE_RATE: u128 = 1000;

// enemy globals
//pub const STUN_TIME: u32 = 2000;
pub const FIRE_COOLDOWN_E: u128 = 2500;

pub struct GameData {
    pub frame_counter: Instant, 
    speed_limit: f64,
    accel_rate: f64,

    pub gold: Vec<Gold>,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub crates: Vec<Crate>,
    
    pub current_floor: i32, 
    pub current_room: usize, // used to keep track of the room the player is in once we have multiple rooms
    pub rooms: Vec<Room>,
}

impl GameData {
    pub fn new() -> GameData {
        // creating a level: room data
        let current_floor = 1; // starting floor
        let current_room = 0; // starting room
        let mut rooms: Vec<Room> = Vec::with_capacity(rand::thread_rng().gen_range(8..11));
        let mut i = 0;
        while i < rooms.capacity() {
            rooms.push(Room::new());
            i += 1;
        }

        // global values:
        let speed_limit = 3.0;
        let accel_rate = 0.0;

        // objects
        let gold: Vec<Gold> = Vec::with_capacity(5);
        let player_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let enemy_projectiles: Vec<Projectile> = Vec::with_capacity(4);
        let crates: Vec<Crate> = Vec::<Crate>::with_capacity(5);
        let frame_counter = Instant::now();

        GameData {
            current_floor, 
            frame_counter, 
            current_room,
            gold,
            player_projectiles,
            enemy_projectiles,
            rooms,
            speed_limit,
            accel_rate,
            crates,
        }
    }

    // speed values
    pub fn set_speed_limit(&mut self, speed_limit: f64) {
        //println!("Speed limit adjusted: {}", speed_limit);
        self.speed_limit = speed_limit;
    }

    pub fn get_speed_limit(&self) -> f64 {
        self.speed_limit
    }

    pub fn set_accel_rate(&mut self, accel_rate: f64) {
        //println!("Acceleration rate adjusted: {}", accel_rate);
        self.accel_rate = accel_rate;
    }

    pub fn get_accel_rate(&self) -> f64 {
        self.accel_rate
    }

    // collisions
    pub fn check_collision(a: &Rect, b: &Rect) -> bool {
        // check collision
        if a.bottom() <= b.top()
            || a.top() >= b.bottom()
            || a.right() <= b.left()
            || a.left() >= b.right()
        {
            false
        } else {
            true
        }
    }
}
