extern crate sdl2;

mod credits;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

const TITLE: &str = "Roguelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const FRAME_GAP: u32 = 200;

pub fn main() -> Result<(), String> {
    credits::run_credits();
    Ok(())
}
