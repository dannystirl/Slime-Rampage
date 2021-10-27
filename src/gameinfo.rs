use sdl2::rect::Rect;
use rand::Rng;

use crate::projectile::*;
use crate::gold::*;
use crate::room::*;

const TILE_SIZE: u32 = 64;

pub struct GameData{
    pub gold: Vec<Gold>,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub current_room: i32, // used to keep track of the room the player is in once we have multiple rooms
    pub rooms: vec![],
    speed_limit: f64, 
    accel_rate: f64, 
}

impl GameData{
    pub fn new(room: Room) -> GameData{
        // creating a level: room data
        let current_room = 0; // starting room
        let num_rooms = rand::thread_rng().gen_range(8..11);
        let mut rooms = vec![room]; // creating as a vec! instead of Vec for indexing, which then requires an initial value. if init as Vec, don't pass room into GameData
        let mut i=0;
        while i < num_rooms {
            let room = Room::new();
            rooms[i] = room;
        }
        println!("{}", rooms.capacity());

        // global values: 
        let speed_limit = 3.0;
		let accel_rate = 0.0;

        // objects
        let mut gold: Vec<Gold> = Vec::with_capacity(5);
        let mut player_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let mut enemy_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        GameData{
            current_room, 
            gold,
            player_projectiles,
            enemy_projectiles, 
            rooms, 
            speed_limit, 
            accel_rate, 
        }
    }

    // speed values
    pub fn set_speed_limit(&self, speed_limit: f64){
        //println!("Speed limit adjusted: {}", speed_limit);
        self.speed_limit = speed_limit;
    }

    pub fn get_speed_limit(&self) -> f64 {
        self.speed_limit
    }

    pub fn set_accel_rate(&self, accel_rate: f64){
        //println!("Acceleration rate adjusted: {}", accel_rate);
        self.accel_rate = accel_rate;
    }

    pub fn get_accel_rate(&self) -> f64 {
        self.accel_rate
    }

    // collisions
    pub fn check_collision(a: &Rect, b: &Rect) -> bool { // check collision
        if a.bottom() < b.top()
            || a.top() > b.bottom()
            || a.right() < b.left()
            || a.left() > b.right()
        { false }
        else { true }
    }
}

