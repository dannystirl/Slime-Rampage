extern crate rogue_sdl;

use crate::gamedata::*;

use sdl2::rect::Rect;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PowerType {
    Fireball,
    Slimeball,
    Rock,
    Shrapnel,
    Shield,
    Dash,
    None,
}

#[derive(Copy, Clone)]
pub struct Power {
    pos: Rect,
    src: Rect,
    pub power_type: PowerType,
    collected: bool,
    pub damage: i32, 
    pub mana_cost: i32,
    pub flammable: bool, 
}

impl Power {
    pub fn new(pos: Rect, power_type: PowerType) -> Power {
        let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        let collected = false;
        let damage: i32; 
        let mana_cost: i32; 
        let flammable: bool; 
        match power_type {
            PowerType::Fireball => { damage = 5; mana_cost = 3; flammable = true; }
            PowerType::Slimeball => { damage = 3; mana_cost = 2; flammable = false; }
            PowerType::Rock => { damage = 8; mana_cost = 4; flammable = true; }
            PowerType::Shrapnel => { damage = 3; mana_cost = 0; flammable = true; }
            PowerType::Shield => { damage = 0; mana_cost = 4; flammable = false; }
            PowerType::Dash => { damage = 0; mana_cost = 3; flammable = false; }
            _ => { damage = 2; mana_cost = 3; flammable = false; }
        }
        Power {
            pos,
            src,
            power_type,
            collected,
            damage, 
            mana_cost, 
            flammable, 
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

    pub fn collected(&self) -> bool {
        self.collected
    }

    pub fn set_collected(&mut self) {
        self.collected = true;
    }

    pub fn power_type(&self) -> &PowerType {
        &self.power_type
    }

    pub fn upgrade_power_damage(&mut self, damage: i32) {
        self.damage += damage; 
    }
}