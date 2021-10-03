extern crate sdl2;

use sdl2::event::Event;
use std::time::Duration;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;

const TITLE: &str = "Roguelike Credits";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 4500;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                // These keypresses aren't working...can't figure out why!
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'credits_loop,
                Event::KeyDown{keycode: Some(Keycode::Q), ..} => break 'credits_loop,
                _ => {},
            }
            // Copy image texture to canvas, present, timeout
    
            // Title
            let mut texture = texture_creator.load_texture("images/credits/credits_title.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(TIMEOUT));
    
            // Maybe split up people by teams

            // Davon
            texture = texture_creator.load_texture("images/credits/credits_davon.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(TIMEOUT));

            // Daniel 
            texture = texture_creator.load_texture("images/credits/credits_daniel.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(TIMEOUT));

            //Victor
            texture = texture_creator.load_texture("images/credits/credits_victor.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(TIMEOUT));

            //Adam
            texture = texture_creator.load_texture("images/credits/credits_adam.png")?;
            canvas.copy(&texture, None, None)?;
            canvas.present();
            ::std::thread::sleep(Duration::from_millis(TIMEOUT));

            break 'credits_loop;
        }
    }

    Ok(())
}
