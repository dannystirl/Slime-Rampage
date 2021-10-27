
use crate::projectile::*;
use crate::gold::*;

pub struct GameData{
    pub projectiles: Vec<Projectile>,
    pub gold: Vec<Gold>,
}

impl GameData{
    pub fn new() -> GameData{
        let mut projectiles: Vec<Projectile> = Vec::with_capacity(5);
        let mut gold: Vec<Gold> = Vec::with_capacity(5);
        GameData{
            projectiles,
            gold,
        }
    }
}