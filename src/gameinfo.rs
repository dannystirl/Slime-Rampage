
use crate::projectile::*;

pub struct GameData{
    pub projectiles: Vec<Projectile>,
}

impl GameData{
    pub fn new() -> GameData{
        let projectiles: Vec<Projectile> = Vec::with_capacity(5);
        GameData{
            projectiles,
        }
    }

}