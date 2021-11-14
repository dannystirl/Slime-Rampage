extern crate rogue_sdl;

use crate::gamedata::*;

use sdl2::rect::Rect;

pub enum PowerType {
    None,
    Fireball,
    Slimeball,
}

pub struct Power {
    pos: Rect,
    src: Rect,
    power_type: PowerType,
}

impl Power {
    pub fn new(pos: Rect, power_type: PowerType) -> Power {
        let src = Rect::new(0 as i32, 0 as i32, TILE_SIZE, TILE_SIZE);
        Power {
            pos,
            src,
            power_type,
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

    pub fn power_type(&self) -> &PowerType {
        &self.power_type
    }
}