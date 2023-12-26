use rand;
use rand::Rng;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use crate::soundfx::SoundEffect;

use crate::letter_bricks::LetterBricks;
use crate::common::{ScreenObjectArea, SCREEN_HEIGHT};
use crate::image::Image;
use crate::animation::{AnimationSeq, Animation, Animations};

const START_Y: i32 = 120;
const BOMB_WIDTH: u32 = 50;
const BOMB_HEIGHT: u32 = 22;
const BOMB_SPEED: i32 = 2;
const BOMB_PERIOD: u32 = 10;
const BOMB_VALUE: [usize; 3] = [10, 30, 50];
const SCORE_MULTIPLIER: u32 = 20;

#[derive(Copy, Clone)]
enum BombType {
    Bonus10,
    Bonus30,
    Bonus50,
}

enum State {
    Dormant,
    InFlight,
}

pub struct BonusBomb<'a> {
    x: i32,
    y: i32,
    bomb_state: State,
    bomb_type: BombType,
    bomb_image: [[Image<'a>; 2]; 3],
    bonus_bomb_sound: SoundEffect,
    bonus_bomb_hit_sound: SoundEffect,
    sound_on: bool,
}

impl<'a> BonusBomb<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> BonusBomb {
        BonusBomb {
            x: 0,
            y: 0,
            bomb_state: State::Dormant,
            bomb_type: BombType::Bonus10,
            bomb_image: [[Image::new(texture_creator, "bonus10_1.png", BOMB_WIDTH, BOMB_HEIGHT),
                          Image::new(texture_creator, "bonus10_2.png", BOMB_WIDTH, BOMB_HEIGHT)],
                         [Image::new(texture_creator, "bonus30_1.png", BOMB_WIDTH, BOMB_HEIGHT),
                          Image::new(texture_creator, "bonus30_2.png", BOMB_WIDTH, BOMB_HEIGHT)],
                         [Image::new(texture_creator, "bonus50_1.png", BOMB_WIDTH, BOMB_HEIGHT),
                          Image::new(texture_creator, "bonus50_2.png", BOMB_WIDTH, BOMB_HEIGHT)]],
            bonus_bomb_sound: SoundEffect::new("bonus_bomb.ogg"),
            bonus_bomb_hit_sound: SoundEffect::new("spider_explosion.ogg"),   // for now
            sound_on: false,
        } 
    }

    fn area(&self) -> ScreenObjectArea {
        // only the middle part of the bonus bomb is collidable
        ScreenObjectArea::new(self.x + 10, self.y, BOMB_WIDTH - 20, BOMB_HEIGHT)
    }

    pub fn in_flight(&self) -> bool {
        match self.bomb_state {
            State::InFlight => {true},
            _ => {false}
        }
    }

    pub fn reset(&mut self) {
        self.bomb_state = State::Dormant;
    }

    pub fn launch(&mut self, x: i32) {
        self.x = x - BOMB_WIDTH as i32 / 2;
        self.y = START_Y;
        self.bomb_state = State::InFlight;
        self.bomb_type =
            match rand::thread_rng().gen_range(0..3) {
                0 => {BombType::Bonus10},
                1 => {BombType::Bonus30},
                _ => {BombType::Bonus50},
            };
    }

    pub fn collision(&mut self, col_area: ScreenObjectArea) -> bool {
        self.in_flight() && col_area.collides(self.area())
    }

    pub fn achieve_bonus(&mut self, letter_bricks: &mut LetterBricks, animations: &mut Animations) {
        self.bomb_state = State::Dormant;
        letter_bricks.initiate_removal(BOMB_VALUE[self.bomb_type as usize]);

        let x = self.x + BOMB_WIDTH as i32 / 2;
        let y = self.y + BOMB_HEIGHT as i32 / 2;
        let bomb_type = self.bomb_type as usize;
        let animation = Animation::new(AnimationSeq::BonusBombHit(x, y, bomb_type));
        animations.register(animation);
        self.bonus_bomb_hit_sound.play();
    }

    pub fn score(&self) -> u32 {
        BOMB_VALUE[self.bomb_type as usize] as u32 * SCORE_MULTIPLIER
    }

    pub fn turn_sound_on(&mut self) {
        self.sound_on = true;
    }
   
    pub fn turn_sound_off(&mut self) {
        self.sound_on = false;
    }

    pub fn update(&mut self) {
        match self.bomb_state {
            State::InFlight => {
                if (self.y - START_Y) % 120 == 0 && self.sound_on {
                    self.bonus_bomb_sound.play();
                }
                self.y += BOMB_SPEED;
                if self.y > SCREEN_HEIGHT as i32 {
                    self.bomb_state = State::Dormant;
                }
            },
            _ => {}
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, frame_count: u32) {
        let bt = self.bomb_type as usize;
        let an = ((frame_count / BOMB_PERIOD) % 2) as usize;

        match self.bomb_state {
            State::InFlight => {
                self.bomb_image[bt][an].render(canvas, self.x, self.y);
            },
            _ => {},
        }
    }
}
