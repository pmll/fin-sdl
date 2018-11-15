use rand;
use rand::Rng;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use common::TargetBrick;
use image::Image;

const BASE_BRICKS_Y: i32 = 630;
const BRICKS_HOME_X: i32 = 60;
const BRICKS_SPEED: i32 = 2;
const BRICK_HEIGHT: u32 = 10;
const BRICK_WIDTH: u32 = 15;
const BRICK_QTY: usize = 4 * 4 * 3;

pub struct BaseBricks<'a> {
    x: i32,
    filled: [bool; 4 * 4 * 3],
    targetted: [bool; 4 * 4 * 3],
    qty_filled: u32,
    brick_image: Image<'a>,
}

impl<'a> BaseBricks<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> BaseBricks {
        BaseBricks {x: 60, filled: [false; BRICK_QTY], targetted: [false; BRICK_QTY],
            qty_filled: 0, brick_image: Image::new(texture_creator, "brick.png", BRICK_WIDTH - 2, BRICK_HEIGHT - 2)}
    }

    pub fn reset(&mut self) {
        self.qty_filled = 0;
    }

    fn brick_id(pile: usize, col: usize, row: usize) -> usize {
        pile * 16 + row * 4 + col
    }

    fn row_has_bricks(&self, pile: usize, row: usize) -> bool {
        let i = pile * 16 + row * 4;
        self.filled[i] || self.filled[i + 1] || self.filled[i + 2] || self.filled[i + 3]
    }

    pub fn update(&mut self) {
        if self.qty_filled == 0 {
            self.x = -600;
            self.filled = [true; BRICK_QTY];
            self.targetted = [false; BRICK_QTY];
            self.qty_filled = BRICK_QTY as u32;
        }
        else if self.x < BRICKS_HOME_X {
             self.x += BRICKS_SPEED;
        }
    }

    pub fn request_target(&mut self) -> Option<TargetBrick> {
        // build up a list of all bricks that are in the top row of their pile that
        // are not already targetted and are not next to another that is targetted
        let mut target_list = [0; 12];
        let mut list_len: usize = 0;
        if self.qty_filled > 0 && self.x >= BRICKS_HOME_X {
            for i in 0..3 {
                for j in 0..4 {
                    if self.row_has_bricks(i, j) {
                        let id0 = BaseBricks::brick_id(i, 0, j);
                        let t0 = self.targetted[id0];
                        let t1 = self.targetted[id0 + 1];
                        let t2 = self.targetted[id0 + 2];
                        let t3 = self.targetted[id0 + 3];
                        if (! t0) && (! t1) && self.filled[id0] {
                            target_list[list_len] = id0;
                            list_len += 1;
                        }
                        if (! t0) && (! t1) && (! t2) && self.filled[id0 + 1] {
                            target_list[list_len] = id0 + 1;
                            list_len += 1;
                        }
                        if (! t1) && (! t2) && (! t3) && self.filled[id0 + 2] {
                            target_list[list_len] = id0 + 2;
                            list_len += 1;
                        }
                        if (! t2) && (! t3) && self.filled[id0 + 3] {
                            target_list[list_len] = id0 + 3;
                            list_len += 1;
                        }
                        break;
                    }
                }
            }
        }
        // choose a random target from the list
        if list_len > 0 {
            let id = target_list[rand::thread_rng().gen_range(0, list_len)];
            let x = self.x + (id as i32 / 16) * 210 + (id as i32 % 4) * BRICK_WIDTH as i32;
            let y = BASE_BRICKS_Y + ((id as i32 % 16) / 4) * BRICK_HEIGHT as i32;
            self.targetted[id] = true;
            Some(TargetBrick {x, y, brick_id: id})
        }
        else {
            None
        }
    }

    pub fn take_target(&mut self, brick_id: usize) {
        self.filled[brick_id] = false;
        self.targetted[brick_id] = false;
        self.qty_filled -= 1;
    }

    pub fn untarget(&mut self, brick_id: usize) {
        self.targetted[brick_id] = false;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for i in 0..3 {
            for j in 0..4 {
                for k in 0..4 {
                    if self.filled[BaseBricks::brick_id(i, j, k)] {
                        let y = BASE_BRICKS_Y + (k as i32) * BRICK_HEIGHT as i32;
                        let x = self.x + i as i32 * 210 + j as i32 * BRICK_WIDTH as i32;
                        self.brick_image.render(canvas, x, y);
                    }
                }
            }
        }
    }
}
