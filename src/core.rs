extern crate rogue_sdl;
mod enemy;
mod background;
mod player;
mod ui;
mod projectile;
mod credits;
mod gameinfo;
mod gold;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
//use std::time::Duration;
//use std::time::Instant;
use rand::Rng;
use crate::enemy::*;
use crate::projectile::*;
use crate::player::*;
use crate::background::*;
use crate::ui::*;
use crate::gold::*;

use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState};
//use sdl2::mouse::MouseButtonIterator;
//use sdl2::mouse::PressedMouseButtonIterator;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
//use sdl2::render::WindowCanvas;
//use sdl2::render::Texture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::render::TextureQuery;


use rogue_sdl::{Game, SDLCore};
use sdl2::video::WindowContext;
use crate::gameinfo::GameData;

pub struct ROGUELIKE {
	core: SDLCore,
	game_data: GameData,
}
impl 
fn init() -> Result<Self, String> {
    let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
    let game_data = GameData::new();
    Ok(ROGUELIKE{ core, game_data })
}