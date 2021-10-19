extern crate rogue_sdl;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;


use rogue_sdl::SDLCore;
use rogue_sdl::Game;


const TITLE: &str = "Top Down Scrolling";

const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const BG_W: u32 = 1920;
const BG_H: u32 = 1080;

const TILE_SIZE: u32 = 100;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

pub struct Scroll 
{
	core: SDLCore,
}

fn update_player(mut player: &mut Player )
{
    let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

    let mut x_deltav = 0;
    let mut y_deltav = 0;

    
}