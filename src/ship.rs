
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use crate::common::{SCREEN_WIDTH, SCREEN_HEIGHT, UPDATE_FPS, ScreenObjectArea};
use crate::missile::Missile;
use crate::image::Image;
use crate::soundfx::SoundEffect;
use crate::animation::{AnimationSeq, Animation, Animations};

const SHIP_WIDTH: u32 = 30;
const SHIP_HEIGHT: u32 = 40;
const SHIP_Y: i32 = 565;
const SHIP_SPEED: i32 = 5;
const LIVES: u32 = 4;
const LIVES_Y: i32 = (SCREEN_HEIGHT - 2 - SHIP_HEIGHT / 2) as i32;
const LIVES_X: i32 = (SCREEN_WIDTH - 2 - SHIP_WIDTH / 2) as i32;
const MIN_FRAMES_BEFORE_CHANGEOVER: u32 = UPDATE_FPS as u32 * 3 / 2;
const GRACE_PERIOD_FRAMES: u32 = UPDATE_FPS as u32;

enum ShipState {
    Alive(u32),
    WaitForChangeOver(u32),
    ChangeOver(f64),
}

pub struct Ship<'a> {
    x: i32,
    state: ShipState,
    lives: u32,
    ship_image: [Image<'a>; 3],
    ship_explosion_sound: SoundEffect,
    extra_life_sound: SoundEffect,
}

impl<'a> Ship<'a> {
    pub fn new (texture_creator: &TextureCreator<WindowContext>) -> Ship {
        Ship{x: Ship::home_x(),
             state: ShipState::Alive(0),
             lives: LIVES,
             ship_image: [Image::new(texture_creator, "ship1.png", SHIP_WIDTH, SHIP_HEIGHT),
                          Image::new(texture_creator, "ship2.png", SHIP_WIDTH, SHIP_HEIGHT),
                          Image::new(texture_creator, "ship3.png", SHIP_WIDTH, SHIP_HEIGHT)],
             ship_explosion_sound: SoundEffect::new("ship_explosion.ogg"),
             extra_life_sound: SoundEffect::new("extra_life.ogg")}
    }

    fn home_x() -> i32 {
        ((SCREEN_WIDTH - SHIP_WIDTH) / 2) as i32
    }

    pub fn reset(&mut self) {
        self.x = Ship::home_x();
        self.state = ShipState::Alive(0);
        self.lives = LIVES;
    }

    pub fn move_left(&mut self) {
        if let ShipState::Alive(_) = self.state {
            if self.x > 0 {
                self.x -= SHIP_SPEED;
            }
        }
    }

    pub fn move_right(&mut self) {
        if let ShipState::Alive(_) = self.state {
            if self.x < (SCREEN_WIDTH - SHIP_WIDTH) as i32 {
                self.x += SHIP_SPEED;
            }
        }
    }

    pub fn kill(&mut self, animations: &mut Animations) {
        if let ShipState::Alive(_) = self.state {
            let x = self.x;
            let animation = Animation::new(AnimationSeq::ShipExplosion(x, SHIP_Y));
            animations.register(animation);
            self.ship_explosion_sound.play();
            self.state = ShipState::WaitForChangeOver(MIN_FRAMES_BEFORE_CHANGEOVER);
            self.x = Ship::home_x();
        }
    }

    pub fn award_extra_life(&mut self, animations: &mut Animations) {
        self.lives += 1;
        let animation = Animation::new(AnimationSeq::ExtraLife);
        animations.register(animation);
        self.extra_life_sound.play();
    }

    pub fn alive(&self) -> bool {
        if let ShipState::Alive(_) = self.state {true} else {false}
    }

    pub fn life_left(&self) -> bool {
        if self.lives == 0 {
            if let ShipState::WaitForChangeOver(_) = self.state {
                return false;
            }
        }
        return true;
    }

    pub fn area(&self) -> ScreenObjectArea {
        ScreenObjectArea::new(self.x, SHIP_Y, SHIP_WIDTH, SHIP_HEIGHT)
    }

    pub fn update(&mut self) {
        match self.state {
            ShipState::WaitForChangeOver(n) => {
                if n > 0 {
                    self.state = ShipState::WaitForChangeOver(n - 1);
                }
            }
            ShipState::ChangeOver(n) => {
                if n < 1.0 {
                    self.state = ShipState::ChangeOver(n + 0.05);
                }
                else {
                    self.state = ShipState::Alive(GRACE_PERIOD_FRAMES);
                }
            },
            ShipState::Alive(n) => {
                if n > 0 {
                    self.state = ShipState::Alive(n - 1);
                }
            },
        }
    }

    pub fn launch_missile(&self, missile: &mut Missile) {
        if let ShipState::Alive(_) = self.state {
            missile.launch(self.x + (SHIP_WIDTH / 2) as i32, SHIP_Y);
        }
    }

    pub fn waiting_for_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(_) => {true},
            _ => {false},
        }
    }

    pub fn enough_delay_for_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(n) => {n == 0},
            _ => {false}
        }
    }

    pub fn in_changeover(&self) -> bool {
        match self.state {
            ShipState::WaitForChangeOver(_) => {true},
            ShipState::ChangeOver(_) => {true},
            _ => {false},
        }
    }

    pub fn proceed_with_changeover(&mut self) {
        if self.lives > 0 {
            self.state = ShipState::ChangeOver(0.0);
            self.lives -= 1;
        }
    }

    pub fn protected(&self) -> bool {
        match self.state {
            ShipState::Alive(n) => {n > 0},
            _ => {false},
        }
    }

    fn life_x(life: u32) -> i32 {
        LIVES_X - (life * (SHIP_WIDTH / 2 + 10)) as i32
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, frame_count: u32) {
        match self.state {
            ShipState::Alive(_) => {
                let ship_pulse = frame_count % 30;
                self.ship_image[(ship_pulse / 10) as usize].render(canvas, self.x, SHIP_Y);
            },
            ShipState::ChangeOver(n) => {
                self.ship_image[0].render_resize(canvas, self.x + ((Ship::life_x(self.lives + 1) - self.x) as f64 * (1.0 - n)) as i32, SHIP_Y + ((LIVES_Y - SHIP_Y) as f64 * (1.0 - n)) as i32, 0.5 + 0.5 * n);
            },
            _ => {},
        }
        for i in 0..self.lives {
            self.ship_image[0].render_resize(canvas, Ship::life_x(i), LIVES_Y, 0.5);
            
        }
    }
}

