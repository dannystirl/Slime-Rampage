extern crate rogue_sdl;

use crate::gamedata::*;

use sdl2::rect::Rect;

pub enum WeaponType {
    Sword,
    Spear,
}

pub struct Weapon {
    pos: Rect,
    src: Rect,
    weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(pos: Rect, weapon_type: WeaponType) -> Weapon {
        let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        Weapon {
            pos,
            src,
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

    pub fn set_weapon_type(&mut self, weapon: WeaponType) {
        self.weapon_type = weapon;
    }
}