use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use common::{SCREEN_WIDTH, SCREEN_HEIGHT};
use image::Image;

const FRAMES: usize = 6;
const SCROLL_SPEED: i32 = 1;
const CYCLE_PERIOD: u32 = 10;

pub struct Background<'a> {
    background_image: [Image<'a>; FRAMES],
    frame: u32,
    y: i32,
}

impl<'a> Background<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Background {
        Background {
            background_image: [
                Image::new(texture_creator, "stars-0.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                Image::new(texture_creator, "stars-1.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                Image::new(texture_creator, "stars-2.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                Image::new(texture_creator, "stars-3.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                Image::new(texture_creator, "stars-4.png", SCREEN_WIDTH, SCREEN_HEIGHT),
                Image::new(texture_creator, "stars-5.png", SCREEN_WIDTH, SCREEN_HEIGHT),
            ],
            frame: 0,
            y: 0,
        }
    }
        
    pub fn update(&mut self) {
        self.frame += 1;
        self.y += SCROLL_SPEED;
        if self.y >= SCREEN_HEIGHT as i32 {
            self.y -= SCREEN_HEIGHT as i32;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let anim_frame = (self.frame / CYCLE_PERIOD) as usize % FRAMES;
        self.background_image[anim_frame].render(canvas, 0, self.y - SCREEN_HEIGHT as i32);
        self.background_image[anim_frame].render(canvas, 0, self.y);
    }
}
