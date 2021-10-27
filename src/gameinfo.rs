
use crate::projectile::*;
use crate::gold::*;

pub struct GameData{
    pub gold: Vec<Gold>,
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
}

impl GameData{
    pub fn new() -> GameData{
        let mut gold: Vec<Gold> = Vec::with_capacity(5);
        let mut player_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let mut enemy_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        GameData{
            gold,
            player_projectiles,
            enemy_projectiles, 
        }
    }
}