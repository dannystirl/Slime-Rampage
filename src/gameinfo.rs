
use crate::projectile::*;

pub struct GameData{
    pub projectiles: Vec<Projectile>,
}

impl GameData{
    pub fn new() -> GameData{
        let mut projectiles: Vec<Projectile> = Vec::with_capacity(0);
        
        GameData{
            projectiles,
        }
    }

}