
use crate::projectile::*;

pub struct GameData{
    pub player_projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
}

impl GameData{
    pub fn new() -> GameData{
        let mut player_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let mut enemy_projectiles: Vec<Projectile> = Vec::with_capacity(5);
        GameData{
            player_projectiles,
            enemy_projectiles,
        }
    }
}