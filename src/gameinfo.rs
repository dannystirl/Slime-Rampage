use crate::projectile::*;
use crate::gold::*;
use crate::room::*;
use rand::Rng;

const TILE_SIZE: u32 = 64;

pub struct GameData{
    pub gold: Vec<Gold>,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub current_room: i32, // used to keep track of the room the player is in once we have multiple rooms
}

impl GameData{
    pub fn new() -> GameData{
        let current_room = 0; // starting room
        let mut gold: Vec<Gold> = Vec::with_capacity(5);
        let mut player_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let mut enemy_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        GameData{
            current_room, 
            gold,
            player_projectiles,
            enemy_projectiles, 
        }
    }
}