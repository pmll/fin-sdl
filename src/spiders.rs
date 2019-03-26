use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use rand;
use rand::Rng;
use std::f64::consts::PI;

use common;
use mother::Mother;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use bombs::Bombs;
use image::Image;
use soundfx::SoundEffect;
use animation::{AnimationSeq, Animation, Animations};

// for now, all the spider co-ords/speeds to be kept as float and can be reviewed later
const NUMBER_OF_SPIDERS: usize = 45;
const INIT_IN_FLIGHT: u32 = 7;
const MAX_IN_FLIGHT: u32 = 18;
const SPIDER_WIDTH: f64 = 30.0;
const SPIDER_HEIGHT: f64 = 40.0;
const SPIDER_PERIOD: u32 = 20;
const SPIDER_ROTATE_SPEED: f64 = 0.05;
const SWOOP_SPEED: f64 = 5.0;
const FRAMES_BETWEEN_LAUNCHES: u32 = 30;
const FIRST_LAUNCH: u32 = 100;
const FLIGHT_SPIDER_Y_MAX: f64 = 480.0;
const FLIGHT_SPIDER_Y_MIN: f64 = 200.0;
const SPEED_SLOW: f64 = 2.0;
const SPEED_MEDIUM: f64 = 3.0;
const SPEED_FAST: f64 = 4.0;
const SPEED: [[(f64, f64); 3]; 3] =
    [[(SPEED_SLOW, SPEED_SLOW), (SPEED_SLOW / 2.0, SPEED_SLOW), (0.0, SPEED_SLOW)],
     [(SPEED_MEDIUM, SPEED_MEDIUM / 2.0), (SPEED_MEDIUM, SPEED_MEDIUM), (SPEED_MEDIUM / 2.0, SPEED_MEDIUM)],
     [(SPEED_FAST, 0.0), (SPEED_FAST, SPEED_FAST / 2.0), (SPEED_FAST, SPEED_FAST)]];
const SPIDER_ASCEND_Y: f64 = 510.0;
const BOMB_RELEASE_MAX_Y: f64 = 410.0;

#[derive(Copy, Clone)]
enum State {
    Nestle,
    Swoop(f64, i32),
    Seek(f64, f64, Option<common::TargetBrick>),
    Descend(common::TargetBrick),
    Grab(f64, f64),
    Ascend,
    Carry(f64, f64, Option<common::TargetBrick>),
    Release(f64, f64),
    Dead,
}

#[derive(Copy, Clone)]
enum Type {
    Slow,
    Medium,
    Fast,
}

enum DirRequired {
    Up,
    Down,
    Any,
}

#[derive(Copy, Clone)]
struct Spider {
    spider_type: Type,
    state: State,
    x: f64,
    y: f64,
    next_dir_change: u32,
    next_bomb_release: u32,
    anim_offset: u32,
}

impl Spider {
    fn new() -> Spider {
        Spider {spider_type: Type::Medium, state: State::Nestle, x: 0.0, y: 0.0,
            next_dir_change: 0, next_bomb_release: 0, anim_offset: 0}
    }

    fn alive(&self) -> bool {
        match self.state {
            State::Dead => {false},
            _ => {true},
        }
    }

    fn launch(&mut self, mother: &Mother) -> bool {
        if let Some(d) = mother.launch_dir() {
            // transform coords from relative to mother to centre of spider
            // ready for swoop
            let (mother_x, mother_y) = mother.location();
            self.x += mother_x as f64 + 0.5 * SPIDER_WIDTH * 0.2;
            self.y += mother_y as f64 - 0.5 * SPIDER_WIDTH * 0.2;
            self.state = State::Swoop(0.0, d);
            true
        }
        else {
            false
        }
    }

    fn trajectory_reaches_target(&self, x_target: f64, y_target: f64, x_vel: f64, y_vel: f64) -> bool {
        (y_target - self.y).signum() == y_vel.signum() &&
        (y_vel < 0.0 || x_vel.abs() <= y_vel.abs()) &&    // not too shallow on downward approach
        (((x_target - self.x) / x_vel) * y_vel + self.y - y_target).abs() < y_vel.abs()
    }

    fn new_trajectory_for_target(&self, x_target: f64, y_target: f64) -> Option<(f64, f64)> {
        let s = self.spider_type as usize;
        for i in 0..3 {
            let (x_vel, y_vel) = SPEED[s][i];
            if self.trajectory_reaches_target(x_target, y_target, x_vel, y_vel) {
                return Some((x_vel, y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, -x_vel, y_vel) {
                return Some((-x_vel, y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, -x_vel, -y_vel) {
                return Some((-x_vel, -y_vel));
            }
            if self.trajectory_reaches_target(x_target, y_target, x_vel, -y_vel) {
                return Some((x_vel, -y_vel));
            }
        }
        return None;
    }

    fn random_sign(n: f64) -> f64 {
        if rand::thread_rng().gen() {-n} else {n}
    }

    fn random_vel(&self, dr: DirRequired) -> (f64, f64) {
        let (x_vel, y_vel) = SPEED[self.spider_type as usize][rand::thread_rng().gen_range(0, 3)];
        match dr {
            DirRequired::Down => {
                let y_vel = if y_vel == 0.0 {SPEED_FAST} else {y_vel};
                (Spider::random_sign(x_vel), y_vel)
            },
            DirRequired::Up => {
                let y_vel = if y_vel == 0.0 {SPEED_FAST} else {y_vel};
                (Spider::random_sign(x_vel), -y_vel)
            },
            DirRequired::Any => {
                (Spider::random_sign(x_vel), Spider::random_sign(y_vel))
            },
        }
    }

    fn aimless_wandering(&mut self, x_vel: f64, y_vel: f64) -> (f64, f64) {
        if self.next_dir_change == 0 {
            self.next_dir_change = rand::thread_rng().gen_range(100, 200);
        }

        self.next_dir_change -= 1;

        let (x_vel, y_vel) = if self.next_dir_change == 0
            {self.random_vel(DirRequired::Any)} else {(x_vel, y_vel)};

        let x_vel = if (self.x + x_vel > common::SCREEN_WIDTH as f64 - SPIDER_WIDTH && x_vel > 0.0) ||
            (self.x + x_vel < 0.0 && x_vel < 0.0)
            {- x_vel} else {x_vel};

        let y_vel = if (self.y + y_vel > FLIGHT_SPIDER_Y_MAX - SPIDER_HEIGHT && y_vel > 0.0) ||
            (self.y + y_vel < FLIGHT_SPIDER_Y_MIN && y_vel < 0.0)
            {- y_vel} else {y_vel};
        (x_vel, y_vel)
    }

    fn drop_bomb(&mut self, bombs: &mut Bombs) {
        if self.y < BOMB_RELEASE_MAX_Y {
            if self.next_bomb_release == 0 {
                self.next_bomb_release = rand::thread_rng().gen_range(50, 200);
            }
            self.next_bomb_release -= 1;
            if self.next_bomb_release == 0 {
                bombs.release((self.x + SPIDER_WIDTH / 2.0) as i32, (self.y + SPIDER_HEIGHT) as i32);
            }
        }
    }

    fn update(&mut self, base_bricks: &mut BaseBricks, letter_bricks: &mut LetterBricks,
        bombs: &mut Bombs, restrict: bool, take_brick_sound: &SoundEffect,
        deposit_brick_sound: &SoundEffect, sound_on: bool) {
        match self.state {
            State::Swoop(n, r) => {
                if n < 1.0 {
                    self.state = State::Swoop(n + SPIDER_ROTATE_SPEED, r);
                    self.x -= (PI + PI * n * r as f64).sin() * SWOOP_SPEED;
                    self.y += (PI + PI * n * r as f64).cos() * SWOOP_SPEED;
                }
                else {
                    // transform coords from centre to top left of spider
                    self.x = (self.x - SPIDER_WIDTH * 0.5).floor();
                    self.y = (self.y - SPIDER_HEIGHT * 0.5).floor();
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Down);
                    self.state = State::Seek(x_vel, y_vel, None);
                }
            },
            State::Seek(x_vel, y_vel, target) => {
                match target {
                    Some(target_brick) => {
                        // caculate adjusted target x,y for top left of spider + a little bit of
                        // descend space
                        let adj_x = target_brick.x as f64 - 9.0;
                        let adj_y = target_brick.y as f64 - 40.0;

                        // restricted mode?
                        if restrict {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Seek(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        // have we reached the target (or close enough)?
                        else if (self.x - adj_x).abs() < x_vel.abs() &&
                           (self.y - adj_y).abs() < y_vel.abs() {
                            self.x = adj_x;
                            self.y = adj_y;
                            self.state = State::Descend(target_brick);
                        }
                        // Are we already on a trajectory to reach the target?
                        else if self.trajectory_reaches_target(adj_x, adj_y, x_vel, y_vel) {
                            // stay as we are!
                            self.x += x_vel;
                            self.y += y_vel;
                        }
                        // can we take a trajectory now that will reach the target?
                        else if let Some((new_x_vel, new_y_vel)) = self.new_trajectory_for_target(adj_x, adj_y) {
                            // change direction
                            self.state = State::Seek(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        else {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Seek(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                    },
                    None => {
                        let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                        let new_target = base_bricks.request_target();
                        self.state = State::Seek(new_x_vel, new_y_vel, new_target);
                        self.x += new_x_vel;
                        self.y += new_y_vel;
                    },
                }
                if ! restrict {
                    self.drop_bomb(bombs);
                }
            },
            State::Descend(target) => {
                self.y += 1.0;
                if self.y >= target.y as f64 - SPIDER_HEIGHT + 8.0 {
                    base_bricks.take_target(target.brick_id);
                    self.state = State::Grab(0.0, if rand::thread_rng().gen() {-1.0} else {1.0});
                    if sound_on {
                        take_brick_sound.play();
                    }
                }
            },
            State::Grab(n, r) => {
                if n < 1.0 {
                    self.state = State::Grab(n + SPIDER_ROTATE_SPEED, r);
                }
                else {
                    self.state = State::Ascend;
                }
            },
            State::Ascend => {
                self.y -= match self.spider_type {
                    Type::Slow => {SPEED_SLOW},
                    Type::Medium => {SPEED_MEDIUM},
                    Type::Fast => {SPEED_FAST}};
                if self.y <= SPIDER_ASCEND_Y {
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Up);
                    self.state = State::Carry(x_vel, y_vel, None);
                }
            },
            State::Carry(x_vel, y_vel, target) => {
                match target {
                    Some(target_brick) => {
                        // caculate adjusted target x,y for top left of spider
                        let adj_x = (target_brick.x - 9) as f64;
                        let adj_y = target_brick.y as f64;

                        // restricted mode?
                        if restrict {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Carry(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        // have we reached the target (or close enough)?
                        else if (self.x - adj_x).abs() < x_vel.abs() &&
                           (self.y - adj_y).abs() < y_vel.abs() {
                            letter_bricks.fill_target(target_brick.brick_id);
                            self.state = State::Release(0.0, if rand::thread_rng().gen() {-1.0} else {1.0});
                            self.x = adj_x;
                            self.y = adj_y;
                            if sound_on {
                                deposit_brick_sound.play();
                            }
                        }
                        // Are we already on a trajectory to reach the target?
                        else if self.trajectory_reaches_target(adj_x, adj_y, x_vel, y_vel) {
                            // stay as we are!
                            self.x += x_vel;
                            self.y += y_vel;
                        }
                        // can we take a trajectory now that will reach the target?
                        else if let Some((new_x_vel, new_y_vel)) = self.new_trajectory_for_target(adj_x, adj_y) {
                            // change direction
                            self.state = State::Carry(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                        else {
                            let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                            self.state = State::Carry(new_x_vel, new_y_vel, target);
                            self.x += new_x_vel;
                            self.y += new_y_vel;
                        }
                    },
                    None => {
                        let (new_x_vel, new_y_vel) = self.aimless_wandering(x_vel, y_vel);
                        let new_target = letter_bricks.request_target();
                        self.state = State::Carry(new_x_vel, new_y_vel, new_target);
                        self.x += new_x_vel;
                        self.y += new_y_vel;
                    },
                }
                if ! restrict {
                    self.drop_bomb(bombs);
                }
            },
            State::Release(n, r) => {
                if n < 1.0 {
                    self.state = State::Release(n + SPIDER_ROTATE_SPEED, r);
                }
                else {
                    let (x_vel, y_vel) = self.random_vel(DirRequired::Down);
                    self.state = State::Seek(x_vel, y_vel, None);
                }
            },
            _ => {}
        }
    }

    fn collision(&self, col_area: &common::ScreenObjectArea) -> bool {
        let collides =  col_area.collides(
            common::ScreenObjectArea::new(self.x as i32, self.y as i32, SPIDER_WIDTH as u32, SPIDER_HEIGHT as u32));

        match self.state {
            State::Seek(_, _, _) => {collides},
            State::Ascend => {collides},
            State::Carry(_, _, _) => {collides},
            State::Release(_, _) => {collides},  // this is not really accurate as rotating 
            _ => {false}
        }
    }
}

pub struct Spiders<'a> {
    spider_image_empty: [[Image<'a>; 4]; 3],
    spider_image_laden: [[Image<'a>; 4]; 3],
    spiders_left: u32,
    spiders_in_flight: u32,
    next_spider_launch: usize,
    last_launch_frame: u32,
    max_spiders_in_flight: u32,
    next_wave_countdown: u32,
    spider: [Spider; NUMBER_OF_SPIDERS],
    take_brick_sound: SoundEffect,
    deposit_brick_sound: SoundEffect,
    spider_explode_sound: SoundEffect,
    sound_on: bool,
}

impl<'a> Spiders<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Spiders {
        let w = SPIDER_WIDTH as u32;
        let h = SPIDER_HEIGHT as u32;
        let mut new_spiders = Spiders {
            spider_image_empty:
                [[Image::new(texture_creator, "spider_empty1_1.png", w, h),
                  Image::new(texture_creator, "spider_empty1_2.png", w, h),
                  Image::new(texture_creator, "spider_empty1_3.png", w, h),
                  Image::new(texture_creator, "spider_empty1_4.png", w, h)],
                 [Image::new(texture_creator, "spider_empty2_1.png", w, h),
                  Image::new(texture_creator, "spider_empty2_2.png", w, h),
                  Image::new(texture_creator, "spider_empty2_3.png", w, h),
                  Image::new(texture_creator, "spider_empty2_4.png", w, h)],
                 [Image::new(texture_creator, "spider_empty3_1.png", w, h),
                  Image::new(texture_creator, "spider_empty3_2.png", w, h),
                  Image::new(texture_creator, "spider_empty3_3.png", w, h),
                  Image::new(texture_creator, "spider_empty3_4.png", w, h)]],
            spider_image_laden:
                [[Image::new(texture_creator, "spider_laden1_1.png", w, h),
                  Image::new(texture_creator, "spider_laden1_2.png", w, h),
                  Image::new(texture_creator, "spider_laden1_3.png", w, h),
                  Image::new(texture_creator, "spider_laden1_4.png", w, h)],
                 [Image::new(texture_creator, "spider_laden2_1.png", w, h),
                  Image::new(texture_creator, "spider_laden2_2.png", w, h),
                  Image::new(texture_creator, "spider_laden2_3.png", w, h),
                  Image::new(texture_creator, "spider_laden2_4.png", w, h)],
                 [Image::new(texture_creator, "spider_laden3_1.png", w, h),
                  Image::new(texture_creator, "spider_laden3_2.png", w, h),
                  Image::new(texture_creator, "spider_laden3_3.png", w, h),
                  Image::new(texture_creator, "spider_laden3_4.png", w, h)]],
            spiders_left: NUMBER_OF_SPIDERS as u32,
            spiders_in_flight: 0,
            next_spider_launch: 0,
            last_launch_frame: 0,
            max_spiders_in_flight: 10, // value will only last for first demo
            next_wave_countdown: 0,
            spider: [Spider::new(); NUMBER_OF_SPIDERS],
            take_brick_sound: SoundEffect::new("grab.ogg"),
            deposit_brick_sound: SoundEffect::new("drop.ogg"),
            spider_explode_sound: SoundEffect::new("spider_explosion.ogg"),
            sound_on: false,};
        for i in 0..NUMBER_OF_SPIDERS {
            if i < 11 {
                new_spiders.spider[i].spider_type = Type::Slow;
            }
            else if i > 35 {
                new_spiders.spider[i].spider_type = Type::Fast;
            }
            // nestle spiders are 6 by 8
            // for nestle spiders, x and y are relative to mother
            new_spiders.spider[i].y = ((i / 15) * 8) as f64 - 16.0;
            new_spiders.spider[i].x = ((i % 15) * 6 + 5) as f64;
            new_spiders.spider[i].anim_offset = rand::thread_rng().gen_range(0, SPIDER_PERIOD);
        }
        new_spiders
    }

    pub fn reset(&mut self, screen: u32) {
        for i in 0..NUMBER_OF_SPIDERS {
            self.spider[i].y = ((i / 15) * 8) as f64 - 16.0;
            self.spider[i].x = ((i % 15) * 6 + 5) as f64;
            self.spider[i].state = State::Nestle;
            self.spider[i].next_dir_change = 0;
            self.spider[i].next_bomb_release = 0;
        }
        self.spiders_left = NUMBER_OF_SPIDERS as u32;
        self.spiders_in_flight = 0;
        self.next_spider_launch = 0;
        self.last_launch_frame = 0;
        self.max_spiders_in_flight =
            (INIT_IN_FLIGHT + screen - 1).min(MAX_IN_FLIGHT);
        self.next_wave_countdown = 0;
    }

    pub fn update(&mut self, mother: &Mother, base_bricks: &mut BaseBricks,
                  letter_bricks: &mut LetterBricks, bombs: &mut Bombs,
                  restrict: bool, frame_count: u32) {
        if self.spiders_in_flight == self.max_spiders_in_flight &&
           self.next_wave_countdown <= 0 {
            self.next_wave_countdown = rand::thread_rng().gen_range(200, 400);
        }
        else if self.spiders_in_flight == 0 {
            self.next_wave_countdown = 0;
        }
        else if self.next_wave_countdown > 0 {
            self.next_wave_countdown -= 1;
        }
        if self.spiders_in_flight < self.max_spiders_in_flight &&
           self.next_wave_countdown <= 0 &&
           self.next_spider_launch < NUMBER_OF_SPIDERS &&
           frame_count > FIRST_LAUNCH &&
           frame_count - self.last_launch_frame >= FRAMES_BETWEEN_LAUNCHES &&
           ! restrict &&
           self.spider[self.next_spider_launch].launch(mother) {
            self.next_spider_launch += 1;
            self.spiders_in_flight += 1;
            self.last_launch_frame = frame_count;
        }
        for s in self.spider.iter_mut().filter(|s| match s.state {State::Dead => {false}, _ => {true}}) {
            s.update(base_bricks, letter_bricks, bombs, restrict,
                &self.take_brick_sound, &self.deposit_brick_sound, self.sound_on);
        }
    }

    pub fn collision(&mut self, col_area: common::ScreenObjectArea) -> Option<usize> {
        for i in 0..NUMBER_OF_SPIDERS {
            if self.spider[i].collision(&col_area) {
                return Some(i);
            }
        }
        return None;
    }

    pub fn target_brick_id(&self, spider_id: usize) -> Option<usize> {
        match self.spider[spider_id].state {
            State::Seek(_, _, target) => {
                if let Some(target_brick) = target {
                    Some(target_brick.brick_id)
                }
                else {
                    None
                }
            },
            State::Carry(_, _, target) => {
                if let Some(target_brick) = target {
                    Some(target_brick.brick_id)
                }
                else {
                    None
                }
            },
            _ => {None},
        }
    }

    pub fn carrying(&self, spider_id: usize) -> bool {
        match self.spider[spider_id].state {
            State::Carry(_, _, _) => {true},
            _ => {false},
        }
    }

    pub fn spider_type(&self, spider_id: usize) -> usize {
        self.spider[spider_id].spider_type as usize
    }

    pub fn kill(&mut self, spider_id: usize, animations: &mut Animations) {
        self.spider[spider_id].state = State::Dead;
        self.spiders_in_flight -= 1;
        self.spiders_left -= 1;
        let x = self.spider[spider_id].x as i32;
        let y = self.spider[spider_id].y as i32;
        let animation = Animation::new(AnimationSeq::SpiderExplosion(x, y));
        animations.register(animation);
        if self.sound_on {
            self.spider_explode_sound.play();
        }
    }

    pub fn spiders_remain(&self) -> bool {
        self.spiders_left > 0
    }

    pub fn clear(&self) -> bool {
        ! self.spider.iter().any(|&s| s.alive() && s.y > FLIGHT_SPIDER_Y_MAX)
    }

    pub fn turn_sound_on(&mut self) {
        self.sound_on = true;
    }
   
    pub fn turn_sound_off(&mut self) {
        self.sound_on = false;
    }

    pub fn render(&self, mother: &Mother, canvas: &mut Canvas<Window>, frame_count: u32) {
        let (mother_x, mother_y) = mother.location();
        for spider in self.spider.iter() {
            let anim_frame = (((frame_count + spider.anim_offset) % SPIDER_PERIOD) /
                (SPIDER_PERIOD / 4)) as usize;
            let type_i = spider.spider_type as usize;
            match spider.state {
                State::Nestle => {
                    let x = spider.x as i32 + mother_x;
                    let y = spider.y as i32 + mother_y - 8;
                    &self.spider_image_empty[type_i][3]
                        .render_angle(canvas, x, y, 180.0, 0.2);
                },
                State::Swoop(n, r) => {
                    let scale = 0.2 + 0.8 * n;
                    let angle = 180.0 + 180.0 * n * r as f64;
                    let x = (spider.x - SPIDER_WIDTH * 0.5 * scale) as i32;
                    let y = (spider.y - SPIDER_HEIGHT * 0.5 * scale) as i32;
                    &self.spider_image_empty[type_i][anim_frame]
                        .render_angle(canvas, x, y, angle, scale);
                },
                State::Seek(_, _, _) => {
                    &self.spider_image_empty[type_i][anim_frame]
                        .render(canvas, spider.x as i32, spider.y as i32);
                },
                State::Descend(_) => {
                    &self.spider_image_empty[type_i][anim_frame]
                        .render(canvas, spider.x as i32, spider.y as i32);
                },
                State::Grab(n, r) => {
                    let angle = 180.0 + 180.0 * n * r;
                    &self.spider_image_laden[type_i][anim_frame]
                        .render_angle(canvas, spider.x as i32, spider.y as i32, angle, 1.0);
                },
                State::Ascend => {
                    &self.spider_image_laden[type_i][anim_frame]
                        .render(canvas, spider.x as i32, spider.y as i32);
                },
                State::Carry(_, _, _) => {
                    &self.spider_image_laden[type_i][anim_frame]
                        .render(canvas, spider.x as i32, spider.y as i32);
                },
                State::Release(n, r) => {
                    let angle = 180.0 + 180.0 * n * r;
                    &self.spider_image_empty[type_i][anim_frame]
                        .render_angle(canvas, spider.x as i32, spider.y as i32, angle, 1.0);
                },
                _ => {},
            };
        }
    }
}
