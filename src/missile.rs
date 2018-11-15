use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use common::ScreenObjectArea;
use soundfx::SoundEffect;
use image::Image;

const MISSILE_WIDTH: u32 = 3;
const MISSILE_HEIGHT: u32 = 15;
const MISSILE_SPEED: i32 = 12;

pub struct Missile<'a> {
    x: i32,
    y: i32,
    in_flight: bool,
    missile_image: Image<'a>,
    fire_sound: SoundEffect,
}

impl<'a> Missile<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Missile {
        Missile {x: 0, y: 0, in_flight: false,
            missile_image: Image::new(texture_creator, "missile.png", MISSILE_WIDTH, MISSILE_HEIGHT),
            fire_sound: SoundEffect::new("fire.ogg")}
    }

    pub fn reset(&mut self) {
        self.in_flight = false;
    }

    pub fn launch(&mut self, from_x: i32, from_y: i32) {
        if ! self.in_flight {
            self.x = from_x - (MISSILE_WIDTH / 2) as i32;
            self.y = from_y - MISSILE_HEIGHT as i32;
            self.in_flight = true;
            self.fire_sound.play();
        }
    }

    pub fn update(&mut self) {
        if self.in_flight {
            self.y -= MISSILE_SPEED;
        }
        if self.y < 0 {
            self.in_flight = false;
        }
    }

    pub fn area(&self) -> ScreenObjectArea {
        ScreenObjectArea::new(self.x, self.y, MISSILE_WIDTH, MISSILE_HEIGHT)
    }

    pub fn flying(&self) -> bool {
        self.in_flight
    }

    pub fn terminate_flight(&mut self) {
        self.in_flight = false;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        if self.in_flight {
            self.missile_image.render(canvas, self.x, self.y);
        }
    }
}

