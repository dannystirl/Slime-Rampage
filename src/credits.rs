extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

const TITLE: &str = "Roguelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const FRAME_GAP: u32 = 200;

pub fn run_credits() -> Result<(), String> {
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

    let mut i = 0;
    'credits_loop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'credits_loop,
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => break 'credits_loop,
                _ => {},
            }
        }
        // Copy image texture to canvas, present, timeout

	let texture;

        match i {
            i if i < FRAME_GAP * 1 => {
		// Title
                texture = texture_creator.load_texture("images/credits/credits_title.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 2 => {
        // Physics Engine Team
                texture = texture_creator.load_texture("images/credits/credits_physics.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 3 => {
		// Davon Allensworth
                texture = texture_creator.load_texture("images/credits/credits_davon.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 4 => {
        // Zirui Huang
                texture = texture_creator.load_texture("images/credits/zih_credit.jpg")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 5 => {
		// Victor Mui
                texture = texture_creator.load_texture("images/credits/credits_victor.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 6 => {
		// Adam Wachowicz
                texture = texture_creator.load_texture("images/credits/credits_adam.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 7 => {
        // Procedural Generation Team
                texture = texture_creator.load_texture("images/credits/credits_procedural_gen.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 8 => {
		// Yihua Pu
                texture = texture_creator.load_texture("images/credits/Yihua_credit.png")?;
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            i if i < FRAME_GAP * 9 => {
		// Marshall Lentz
            texture = texture_creator.load_texture("images/credits/credits_marshall.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            }
            i if i < FRAME_GAP * 10 => {
        // Josh Friedman
            texture = texture_creator.load_texture("images/credits/friedman_credits.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            }
            i if i < FRAME_GAP * 11 => {
        // Daniel Stirling
            texture = texture_creator.load_texture("images/credits/credits_daniel.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
        }
            _ => {}
    }

	i += 1;
        if i > FRAME_GAP * 12 {
            i = 0;
        }
    }
    Ok(())
}
