extern crate sdl2;

use sdl2::event::Event;
use std::time::Duration;
use sdl2::image::LoadTexture;

const TITLE: &str = "Rougelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 4500;

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

    for event in sdl_context.event_pump()?.poll_iter() {
        // Copy image texture to canvas, present, timeout

        // Title
        let texture = texture_creator.load_texture("imagescredits//credits_title.png")?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(TIMEOUT));

        // Maybe split up people by teams

        // Davon
        let texture = texture_creator.load_texture("images/credits/credits_davon.png")?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(TIMEOUT));

        let texture = texture_creator.load_texture("imagescredits//credits_daniel.png")?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(TIMEOUT/28));

        // ...
    }
    Ok(())
}