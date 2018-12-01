// FIN (sdl)
extern crate sdl2;
extern crate find_folder;
extern crate rand;

mod common;
mod ship;
mod base_bricks;
mod letter_bricks;
mod missile;
mod game;
mod bonus_bomb;
mod mother;
mod spiders;
mod bombs;
mod background;
mod soundfx;
mod image;
mod text;
mod animation;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{DEFAULT_CHANNELS, INIT_OGG, AUDIO_S16LSB, Channel};
use sdl2::render::BlendMode::Blend;
use std::time::{Duration, Instant};
use std::thread::sleep;

use game::Game;
use common::{SCREEN_WIDTH, SCREEN_HEIGHT, UPDATE_FPS};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("FIN (sdl)", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(Blend);
    let texture_creator = canvas.texture_creator();

    let mut events = sdl_context.event_pump().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let _audio = sdl_context.audio().unwrap();
    sdl2::mixer::open_audio(
        44_100, // frequency
        AUDIO_S16LSB, //format
        DEFAULT_CHANNELS,
        1_024 // chunk size
    ).unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();
    Channel::all().set_volume(0);

    let mut game = Game::new(&texture_creator, &ttf_context);

    let tick_length = Duration::new(0, 1_000_000_000u32 / UPDATE_FPS);

    'main: loop {
        let now = Instant::now();

        for event in events.poll_iter() {
            game.update_inputs(&event);

            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    }
                },
                _ => {}
            }
        }

        game.update();
        game.render(&mut canvas);

        // we try to stick to the desired timing - if we overrun, things will slooow down
        let delta = now.elapsed();
        if delta < tick_length {
            sleep(tick_length - delta);
        }
        //else {
        //    println!("Overrun {:?}", delta - tick_length);
        //}
    }
}
