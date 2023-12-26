use rand;
use rand::Rng;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use crate::common::SCREEN_WIDTH;
use crate::bonus_bomb::BonusBomb;
use crate::image::Image;

const MOTHER_Y: i32 = 100;
const MOTHER_WIDTH: u32 = 100;
const MOTHER_HEIGHT: u32 = 20;
const MOTHER_PERIOD: u32 = 32;
const MOTHER_SPEED: i32 = 2;

pub struct Mother<'a> {
    x: i32,
    vel: i32,
    mother_image1: Image<'a>,
    mother_image2: Image<'a>,
    bonus_bomb_frame: u32,
}

impl<'a> Mother<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Mother {
        Mother {
            x: (SCREEN_WIDTH - MOTHER_WIDTH) as i32 / 2,
            vel: MOTHER_SPEED,
            mother_image1: Image::new(texture_creator, "mother1.png", MOTHER_WIDTH, MOTHER_HEIGHT),
            mother_image2: Image::new(texture_creator, "mother2.png", MOTHER_WIDTH, MOTHER_HEIGHT),
            bonus_bomb_frame: 0}
    }

    pub fn full_reset(&mut self) {
        self.reset();
        self.x = (SCREEN_WIDTH - MOTHER_WIDTH) as i32 / 2;
        self.vel = MOTHER_SPEED;
    }

    pub fn reset(&mut self) {
        // bonus bomb 20 - 40 sec in
        self.bonus_bomb_frame = 1200 + rand::thread_rng().gen_range(0..1200);
    }

    pub fn update(&mut self, bonus_bomb: &mut BonusBomb, restrict: bool, frame_count: u32) {
        self.x += self.vel;
        if self.x > (SCREEN_WIDTH - MOTHER_WIDTH) as i32 - MOTHER_SPEED ||
           self.x < MOTHER_SPEED {
            self.vel = - self.vel;
        }
        if frame_count == self.bonus_bomb_frame {
            if restrict {
                self.bonus_bomb_frame += 100 + rand::thread_rng().gen_range(0..200);  // postpone it
            }
            else {
                bonus_bomb.launch(self.x + MOTHER_WIDTH as i32 / 2);
            }
        }
    }

    pub fn location(&self) -> (i32, i32) {
        (self.x, MOTHER_Y)
    }

    pub fn launch_dir(&self) -> Option<i32> {
        // to launch a spider with a clean exit, we want to launch in the
        // opposite direction to the mother but only if there is enough space
        // between the mother and the edge of the screen to allow a swoop
        if (self.vel < 0 && self.x < (SCREEN_WIDTH - 50 - MOTHER_WIDTH) as i32) ||
           (self.vel > 0 && self.x > 50) {
            Some(- self.vel.signum())
        }
        else {
            None
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, frame_count: u32) {
        let mother_image = if frame_count % MOTHER_PERIOD < MOTHER_PERIOD / 2
            {&self.mother_image1} else {&self.mother_image2};
        mother_image.render(canvas, self.x, MOTHER_Y);
    }
}
