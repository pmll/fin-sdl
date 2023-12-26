use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use crate::common::{ScreenObjectArea, SCREEN_HEIGHT};
use crate::image::Image;

const BOMB_WIDTH: u32 = 3;
const BOMB_HEIGHT: u32 = 15;
const BOMB_SPEED: i32 = 4;
const MAX_BOMBS: usize = 4;

#[derive(Copy, Clone)]
struct Bomb {
    x: i32,
    y: i32,
    in_flight: bool,
}

impl Bomb {
    fn area(&self) -> ScreenObjectArea {
        ScreenObjectArea::new(self.x, self.y, BOMB_WIDTH, BOMB_HEIGHT)
    }

    fn update(&mut self) {
        if self.in_flight {
            self.y += BOMB_SPEED;
            self.in_flight = self.y < SCREEN_HEIGHT as i32;
        }
    }
}

pub struct Bombs<'a> {
    bomb: [Bomb; MAX_BOMBS],
    bomb_image: Image<'a>,
}

impl<'a> Bombs<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Bombs {
        Bombs {bomb: [Bomb {x: 0, y: 0, in_flight: false}; MAX_BOMBS],
               bomb_image: Image::new(texture_creator, "bomb.png", BOMB_WIDTH, BOMB_HEIGHT)}
    }

    pub fn reset(&mut self) {
        for b in &mut self.bomb {
            b.in_flight = false;
        }
    }

    pub fn release(&mut self, x: i32, y: i32) -> bool {
        for b in self.bomb.iter_mut().filter(|b| ! b.in_flight).take(1) {
            b.x = x - BOMB_WIDTH as i32 / 2;
            b.y = y;
            b.in_flight = true;
            return true;
        }
        return false;
    }

    pub fn collision(&mut self, col_area: ScreenObjectArea) -> bool {
        for b in self.bomb.iter_mut()
            .filter(|b| b.in_flight && col_area.collides(b.area()))
            .take(1) {
            // once bomb has collided, it is no more, take care of it here
            b.in_flight = false;
            return true;
        }
        return false;
    }

    pub fn in_flight(&self) -> bool {
        self.bomb.iter().any(|&b| b.in_flight)
    }

    pub fn update(&mut self) {
        for b in &mut self.bomb {
            b.update();
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for b in self.bomb.iter().filter(|&b| b.in_flight) {
            self.bomb_image.render(canvas, b.x, b.y);
        }
    }
}

