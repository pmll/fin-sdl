use rand;
use rand::Rng;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use common::TargetBrick;
use soundfx::SoundEffect;
use image::Image;

const LETTER_BRICKS_Y: i32 = 140;
const LETTER_BRICKS_X: i32 = 97;
const BRICK_HEIGHT: u32 = 10;
const BRICK_WIDTH: u32 = 15;
const BRICKS_QTY: usize = 86;
const MAX_COMBINED_ROW_BRICKS: usize = 23;
const REMOVE_PERIOD: u32 = 15;

pub struct LetterBrick {
    row: i32,
    col: i32,
    filled: bool,
    targetted: bool,
}

impl LetterBrick {
    fn new(row: i32, col: i32) -> LetterBrick {
        LetterBrick {row, col, filled: false, targetted: false}
    }
}

pub struct LetterBricks<'a> {
    letter_brick: [LetterBrick; BRICKS_QTY],
    letter_range: [[[usize; 2]; 6]; 3],
    to_remove: Vec<usize>,
    qty_filled: usize,
    brick_image: Image<'a>,
    remove_brick_sound: SoundEffect,
    expansion: f64,
    exp_step: f64,
}

impl<'a> LetterBricks<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> LetterBricks {
        LetterBricks {
        letter_brick:
            [
            LetterBrick::new(0, 0), // 0
            LetterBrick::new(0, 1),
            LetterBrick::new(0, 2),
            LetterBrick::new(0, 3),
            LetterBrick::new(0, 4),
            LetterBrick::new(0, 5),
            LetterBrick::new(0, 6),
            LetterBrick::new(0, 7),
            LetterBrick::new(0, 8),
            LetterBrick::new(0, 9),
            LetterBrick::new(0, 11), // 10
            LetterBrick::new(0, 12),
            LetterBrick::new(0, 13),
            LetterBrick::new(0, 14),
            LetterBrick::new(0, 15),
            LetterBrick::new(0, 16),
            LetterBrick::new(0, 18), // 16
            LetterBrick::new(0, 19),
            LetterBrick::new(0, 25),
            LetterBrick::new(0, 26),
            LetterBrick::new(1, 0), // 20
            LetterBrick::new(1, 1),
            LetterBrick::new(1, 13), // 22
            LetterBrick::new(1, 14),
            LetterBrick::new(1, 18), // 24
            LetterBrick::new(1, 19),
            LetterBrick::new(1, 20),
            LetterBrick::new(1, 21),
            LetterBrick::new(1, 25),
            LetterBrick::new(1, 26),
            LetterBrick::new(2, 0), // 30 
            LetterBrick::new(2, 1),
            LetterBrick::new(2, 2),
            LetterBrick::new(2, 3),
            LetterBrick::new(2, 4),
            LetterBrick::new(2, 5),
            LetterBrick::new(2, 6),
            LetterBrick::new(2, 7),
            LetterBrick::new(2, 13), // 38
            LetterBrick::new(2, 14),
            LetterBrick::new(2, 18), // 40
            LetterBrick::new(2, 19),
            LetterBrick::new(2, 20),
            LetterBrick::new(2, 21),
            LetterBrick::new(2, 22),
            LetterBrick::new(2, 25),
            LetterBrick::new(2, 26),
            LetterBrick::new(3, 0), // 47
            LetterBrick::new(3, 1),
            LetterBrick::new(3, 2),
            LetterBrick::new(3, 3),
            LetterBrick::new(3, 4),
            LetterBrick::new(3, 5),
            LetterBrick::new(3, 6),
            LetterBrick::new(3, 7),
            LetterBrick::new(3, 13), // 55
            LetterBrick::new(3, 14),
            LetterBrick::new(3, 18), // 57
            LetterBrick::new(3, 19),
            LetterBrick::new(3, 22),
            LetterBrick::new(3, 23),
            LetterBrick::new(3, 24),
            LetterBrick::new(3, 25),
            LetterBrick::new(3, 26),
            LetterBrick::new(4, 0), // 64
            LetterBrick::new(4, 1),
            LetterBrick::new(4, 13), // 66
            LetterBrick::new(4, 14),
            LetterBrick::new(4, 18), // 68
            LetterBrick::new(4, 19),
            LetterBrick::new(4, 23),
            LetterBrick::new(4, 24),
            LetterBrick::new(4, 25),
            LetterBrick::new(4, 26),
            LetterBrick::new(5, 0), // 74
            LetterBrick::new(5, 1),
            LetterBrick::new(5, 11), // 76
            LetterBrick::new(5, 12),
            LetterBrick::new(5, 13),
            LetterBrick::new(5, 14),
            LetterBrick::new(5, 15),
            LetterBrick::new(5, 16),
            LetterBrick::new(5, 18), // 82
            LetterBrick::new(5, 19),
            LetterBrick::new(5, 25),
            LetterBrick::new(5, 26),
        ],
        letter_range: [[[0, 9], [20, 21], [30, 37], [47, 54], [64, 65], [74, 75]],
                       [[10, 15], [22, 23], [38, 39], [55, 56], [66, 67], [76, 81]],
                       [[16, 19],  [24, 29], [40, 46], [57, 63], [68, 73], [82, 85]]],
        to_remove: Vec::with_capacity(50),
        qty_filled: 0,
        brick_image: Image::new(texture_creator, "letterbrick.png", BRICK_WIDTH - 2, BRICK_HEIGHT - 2),
        remove_brick_sound: SoundEffect::new("remove.ogg"),
        expansion: 0.0, exp_step: 0.0,}
    }

    pub fn reset(&mut self) {
        self.qty_filled = 0;
        for b in &mut self.letter_brick.iter_mut() {
            b.targetted = false;
            b.filled = false;
        }
        self.to_remove.clear();
        self.expansion = 0.0;
        self.exp_step = 0.0;
    }

    fn row_has_gaps(&self, letter: usize, row: usize) -> bool {
        for i in self.letter_range[letter][row][0]..self.letter_range[letter][row][1] + 1 {
            if ! self.letter_brick[i].filled {
                return true;
            }
        }
        return false;
    }

    fn targetted_to_left(&self, id: usize) -> bool {
        id > 0 && self.letter_brick[id - 1].targetted &&
            self.letter_brick[id - 1].col == self.letter_brick[id].col - 1
    }

    fn targetted_to_right(&self, id: usize) -> bool {
        id < BRICKS_QTY - 1 && self.letter_brick[id + 1].targetted &&
            self.letter_brick[id + 1].col == self.letter_brick[id].col + 1
    }

    pub fn fill_target(&mut self, brick_id: usize) {
        self.letter_brick[brick_id].filled = true;
        self.letter_brick[brick_id].targetted = false;
        self.qty_filled += 1;
    }

    pub fn untarget(&mut self, brick_id: usize) {
        self.letter_brick[brick_id].targetted = false;
    }

    pub fn request_target(&mut self) -> Option<TargetBrick> {
        let mut target_list = [0; MAX_COMBINED_ROW_BRICKS];  
        let mut list_len: usize = 0;

        // build a list of gaps in the topmost row of each letter that has
        // unfilled bricks and is not physically next to another targetted gap
        if self.qty_filled < BRICKS_QTY {
            for i in 0..3 {
                for j in 0..6 {
                    if self.row_has_gaps(i, j) {
                        let from = self.letter_range[i][j][0];
                        let to = self.letter_range[i][j][1];
                        for k in from..to + 1 {
                            if ! (self.letter_brick[k].filled || self.letter_brick[k].targetted) &&
                               ! (self.targetted_to_left(k) || self.targetted_to_right(k)) {
                                target_list[list_len] = k;
                                list_len += 1;
                            }
                        }
                        break;
                    }
                }
            }
        }
        // choose a random target from the list
        if list_len > 0 {
            let id = target_list[rand::thread_rng().gen_range(0, list_len)];
            let x = LETTER_BRICKS_X + self.letter_brick[id].col * BRICK_WIDTH as i32;
            let y = LETTER_BRICKS_Y + self.letter_brick[id].row * BRICK_HEIGHT as i32;
            self.letter_brick[id].targetted = true;
            Some(TargetBrick {x, y, brick_id: id})
        }
        else {
            None
        }
    }

    pub fn initiate_removal(&mut self, qty: usize) {
        // build list of filled bricks
        for i in (0..BRICKS_QTY).rev() {
            if self.letter_brick[i].filled {
                self.to_remove.push(i);
                if self.to_remove.len() >= qty {
                    break;
                }
            }
        }
        self.to_remove.reverse();
    }

    pub fn complete(&self) -> bool {
        self.qty_filled == BRICKS_QTY
    }

    pub fn initiate_expansion(&mut self) {
        self.expansion = 1.0;
        self.exp_step = 0.01;
    }

    pub fn update(&mut self, frame_count: u32) {
        if frame_count % REMOVE_PERIOD == 0 {
            if let Some(i) = self.to_remove.pop() {
                self.letter_brick[i].filled = false;
                self.qty_filled -= 1;
                self.remove_brick_sound.play();
            }
        }
        if self.expansion != 0.0 {
            if self.expansion > 1.45 {
                self.exp_step = -0.01;
            }
            else if self.expansion < 1.0 {
                self.exp_step = 0.0;
                self.expansion = 0.0;
            }
            self.expansion += self.exp_step;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        if self.expansion == 0.0 {
            for b in self.letter_brick.iter().filter(|&b| b.filled) {
                let x = LETTER_BRICKS_X + b.col * BRICK_WIDTH as i32;
                let y = LETTER_BRICKS_Y + b.row * BRICK_HEIGHT as i32;
                self.brick_image.render(canvas, x, y);
            }
        }
        else {
            // brick at row 3, column 13 will remain stationary
            let centre_x = LETTER_BRICKS_X + 13 * BRICK_WIDTH as i32;
            let centre_y = LETTER_BRICKS_Y + 3 * BRICK_HEIGHT as i32;

            for b in self.letter_brick.iter() {
               let x = centre_x +
                   ((b.col - 13) as f64 * BRICK_WIDTH as f64 * self.expansion) as i32;
               let y = centre_y +
                   ((b.row - 3) as f64 * BRICK_HEIGHT as f64 * self.expansion) as i32;
               self.brick_image.render(canvas, x, y);
            }
        }
    }
}

