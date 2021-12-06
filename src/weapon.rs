extern crate rogue_sdl;

use crate::gamedata::*;

use sdl2::rect::Rect;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum WeaponType {
    Sword,
    Spear,
    Dagger,
}

pub struct Weapon {
    pos: Rect,
    src: Rect,
    pub attack_cooldown: u128, 
    pub attack_time: u128, 
    pub attack_length: u32, 
    pub damage: i32, 
    pub weapon_type: WeaponType,
}

#[allow(unreachable_patterns)]
impl Weapon {
    pub fn new(pos: Rect, weapon_type: WeaponType) -> Weapon {
        let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let damage: i32; 
        let attack_cooldown: u128;  // cooldown between attacks
        let attack_time: u128;      // time it takes to attack
        let attack_length: u32;     // length of weapons
        match weapon_type {
            WeaponType::Spear => { damage = 6; attack_cooldown = 800; attack_time = 800; attack_length = TILE_SIZE_CAM * 2; }
            WeaponType::Sword => { damage = 3; attack_cooldown = 300; attack_time = 400; attack_length = TILE_SIZE_CAM * 3/2; }
            WeaponType::Dagger => { damage = 2; attack_cooldown = 150; attack_time = 200; attack_length = TILE_SIZE_CAM * 3/4;}
            _ => { damage = 2; attack_cooldown = 300; attack_time = 400; attack_length = TILE_SIZE_CAM * 3/2; }
        }
        Weapon {
            pos,
            src,
            attack_cooldown, 
            attack_time, 
            attack_length, 
            damage, 
            weapon_type,
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x
    }

    pub fn y(&self) -> i32 {
        self.pos.y
    }

    pub fn pos(&self) -> Rect {
        self.pos
    }

    pub fn src(&self) -> Rect {
        self.src
    }

    pub fn weapon_type(&self) -> &WeaponType {
        &self.weapon_type
    }

    pub fn set_weapon_type(&mut self, weapon_type: WeaponType) {
        match weapon_type {
            WeaponType::Spear => { self.damage = 4; self.attack_cooldown = 800; self.attack_length = TILE_SIZE_CAM * 2; }
            WeaponType::Sword => { self.damage = 2; self.attack_cooldown = 300; self.attack_length = TILE_SIZE_CAM * 3/2; }
            WeaponType::Dagger => { self.damage = 2; self.attack_cooldown = 150; self.attack_time = 200; self.attack_length = TILE_SIZE_CAM * 1/2;}
            _ => { self.damage = 2; self.attack_cooldown = 300; self.attack_length = TILE_SIZE_CAM * 3/2; }
        }
    }
}