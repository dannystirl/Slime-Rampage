extern crate sdl2;

mod credits;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

const TITLE: &str = "Roguelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const FRAME_GAP: u32 = 200;

struct Player<'a> {
    texture: Texture<'a>,
}

impl<'a> Player<'a> {
    fn new(texture: Texture<'a>) -> Player {
        Player {
            texture,
        }
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, CAM_W, CAM_H)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // credits::run_credits();

    let texture = texture_creator.load_texture("images/slime_r.png")?;
    canvas.copy(&texture, /*p.src()*/None, /*player_cam_pos*/None)?;

    std::thread::sleep(Duration::from_millis(30000));

    Ok(())
}
