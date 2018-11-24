
use common::{SCREEN_WIDTH, SCREEN_HEIGHT};
use ship::Ship;
use base_bricks::BaseBricks;
use letter_bricks::LetterBricks;
use missile::Missile;
use mother::Mother;
use spiders::Spiders;
use bombs::Bombs;
use background::Background;
use bonus_bomb::BonusBomb;
use soundfx::SoundControl;
use image::Image;
use text::Text;
use animation::{AnimationSeq, Animation, Animations};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Sdl2TtfContext;


const SPIDER_SCORE: [u32; 3] = [40, 80, 200];
const EXTRA_LIFE_SCORE: u32 = 6000;

enum State {
    Startup,
    InProgress,
    GameOver,
}

impl State {
    fn playing(&self) -> bool {
        match *self {
            State::Startup => {false},
            State::GameOver => {false},
            _ => {true},
        }
    }

    fn screen_in_progress(&self) -> bool {
        match *self {
            State::InProgress => {true},
            _ => {false},
        }
    }
}

struct GameInput {
    left_pressed: bool,
    right_pressed: bool,
    fire_pressed: bool,
    start_pressed: bool,
    pause_pressed: bool,
    inc_vol_pressed: bool,
    dec_vol_pressed: bool,
}

impl GameInput {
    fn new() -> GameInput {
        GameInput {
            left_pressed: false,
            right_pressed: false,
            fire_pressed: false,
            start_pressed: false,
            pause_pressed: false,
            inc_vol_pressed: false,
            dec_vol_pressed: false,
        }
    }

    fn reset(&mut self) {
        self.left_pressed = false;
        self.right_pressed = false;
        self.fire_pressed = false;
        self.start_pressed = false;
        self.pause_pressed = false;
        self.inc_vol_pressed = false;
        self.dec_vol_pressed = false;
    }

    fn update_inputs(&mut self, event: &Event) {
        match event {
            Event::KeyDown {keycode: Some(keycode), ..} => {
                match keycode {
                    Keycode::Z => {self.left_pressed = true;},
                    Keycode::X => {self.right_pressed = true;},
                    Keycode::RShift => {self.fire_pressed = true;},
                    Keycode::Space => {self.start_pressed = true;},
                    Keycode::P => {self.pause_pressed = true;},
                    Keycode::Up => {self.inc_vol_pressed = true;},
                    Keycode::Down => {self.dec_vol_pressed = true;},
                    _ => {}
                }
            }

            Event::KeyUp {keycode: Some(keycode), ..} => {
                match keycode {
                    Keycode::Z => {self.left_pressed = false;},
                    Keycode::X => {self.right_pressed = false;},
                    Keycode::Space => {self.start_pressed = false;},
                    _ => {}
                }
            },

            _ => {},
        }
    }

    fn acknowledge_fire(&mut self) {
        self.fire_pressed = false;
    }

    fn acknowledge_pause(&mut self) {
        self.pause_pressed = false;
    }

    fn acknowledge_volume_change(&mut self) {
        self.inc_vol_pressed = false;
        self.dec_vol_pressed = false;
    }
}

pub struct Game<'a, 'b> {
    game_state: State,
    ship: Ship<'a>,
    missile: Missile<'a>,
    base_bricks: BaseBricks<'a>,
    letter_bricks: LetterBricks<'a>,
    mother: Mother<'a>,
    spiders: Spiders<'a>,
    bombs: Bombs<'a>,
    bonus_bomb: BonusBomb<'a>,
    game_over_image: Image<'a>,
    instructions_image: Image<'a>,
    screen_flag_image: Image<'a>,
    game_input: GameInput,
    frame_count: u32,
    score: u32,
    screen: u32,
    sound_control: SoundControl,
    paused: bool,
    animations: Animations<'a, 'b>,
    background: Background<'a>,
    text32: Text<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl2TtfContext) -> Game<'a, 'b> {
        Game {
            game_state: State::Startup,
            ship: Ship::new(texture_creator),
            missile: Missile::new(texture_creator),
            base_bricks: BaseBricks::new(texture_creator),
            letter_bricks: LetterBricks::new(texture_creator),
            mother: Mother::new(texture_creator),
            spiders: Spiders::new(texture_creator),
            bombs: Bombs::new(texture_creator),
            bonus_bomb: BonusBomb::new(texture_creator),
            game_over_image: Image::new(texture_creator, "game_over.png", 427, 72),
            instructions_image: Image::new(texture_creator, "instructions.png", 346, 300),
            screen_flag_image: Image::new(texture_creator, "screen_flag.png", 14, 20),
            game_input: GameInput::new(),
            frame_count: 0,
            score: 0,
            screen: 0,
            sound_control: SoundControl::new(),
            paused: false,
            animations: Animations::new(texture_creator, ttf_context),
            background: Background::new(texture_creator),
            text32: Text::new(ttf_context, 32),
        }
    }

    fn new_game(&mut self) {
        self.game_state = State::InProgress;
        self.mother.full_reset();
        self.screen = 1;
        self.spiders.reset(self.screen);
        self.ship.reset();
        self.missile.reset();
        self.base_bricks.reset();
        self.base_bricks.update();
        self.letter_bricks.reset();
        self.bombs.reset();
        self.bonus_bomb.reset();
        self.score = 0;
        self.frame_count = 0;
        self.game_input.reset();
        self.spiders.turn_sound_on();
        self.bonus_bomb.turn_sound_on();
        self.ship.proceed_with_changeover();
        self.screen_start();
    }

    fn increase_score(&mut self, inc: u32) {
        let q = self.score / EXTRA_LIFE_SCORE;
        self.score += inc;
        if (self.score / EXTRA_LIFE_SCORE) > q {
            self.ship.award_extra_life(&mut self.animations);
        }
    }

    fn render_score(&self, canvas: &mut Canvas<Window>) {
        self.text32.render(canvas, SCREEN_WIDTH as i32 / 2 - 60, 5,
            79, 120, 181, 255, &format!("{:07}", self.score));
    }

    fn screen_start(&mut self) {
        let screen_number = self.screen;
        let animation = Animation::new(AnimationSeq::ScreenStart(screen_number));
        self.animations.register(animation);
    }

    fn render_screens_complete(&self, canvas: &mut Canvas<Window>) {
        for i in 1..self.screen {
            self.screen_flag_image.render(canvas, i as i32 * 20 - 15, SCREEN_HEIGHT as i32 - 22);
        }
    }

    // a collision means occupying the same space in the same frame
    // that may turn out to be too naive but it will do for now
    fn missile_collision(&mut self) {
        if self.missile.flying() {
            if let Some(spider_id) = self.spiders.collision(self.missile.area()) {
                let target_brick_id = self.spiders.target_brick_id(spider_id);
                if let Some(brick_id) = target_brick_id {
                    if self.spiders.carrying(spider_id) {
                        self.letter_bricks.untarget(brick_id);
                    }
                    else {
                        self.base_bricks.untarget(brick_id);
                    }
                }
                self.missile.terminate_flight();
                let points = SPIDER_SCORE[self.spiders.spider_type(spider_id)];
                self.increase_score(points);
                if self.spiders.carrying(spider_id) {
                    self.increase_score(points);
                }
                self.spiders.kill(spider_id, &mut self.animations);
            }
        }
    }

    fn bomb_collision(&mut self) {
        if self.ship.alive() {
            if self.bombs.collision(self.ship.area()) {
                self.ship.kill(&mut self.animations);
            }
        }
    }

    fn spider_collision(&mut self) {
        if self.ship.alive() {
            if let Some(spider_id) = self.spiders.collision(self.ship.area()) {
                let target_brick_id = self.spiders.target_brick_id(spider_id);
                if let Some(brick_id) = target_brick_id {
                    if self.spiders.carrying(spider_id) {
                        self.letter_bricks.untarget(brick_id);
                    }
                    else {
                        self.base_bricks.untarget(brick_id);
                    }
                }
                self.spiders.kill(spider_id, &mut self.animations);
                self.ship.kill(&mut self.animations);
            }
        }
    }

    fn bonus_bomb_collision(&mut self) {
        if self.missile.flying() && self.bonus_bomb.collision(self.missile.area()) {
            self.missile.terminate_flight();
            let points = self.bonus_bomb.score();
            self.increase_score(points);
            self.bonus_bomb.achieve_bonus(&mut self.letter_bricks, &mut self.animations);
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.clear();
        self.background.render(canvas);
        self.render_screens_complete(canvas);
        self.base_bricks.render(canvas);
        self.letter_bricks.render(canvas);
        self.mother.render(canvas, self.frame_count);
        self.spiders.render(&self.mother, canvas, self.frame_count);
        self.bonus_bomb.render(canvas, self.frame_count);
        if self.game_state.playing() {
            self.ship.render(canvas, self.frame_count);
            self.missile.render(canvas);
        }
        self.bombs.render(canvas);
        self.render_score(canvas);
        if let State::GameOver = self.game_state {
            self.game_over_image.render(canvas, 87, 250);
        }
        if ! self.game_state.playing() {
            self.instructions_image.render(canvas, 125, 320);
        }
        self.animations.render(canvas);
        if self.paused {
            self.text32.render(canvas, SCREEN_WIDTH as i32 / 2 - 55, 270,
                0, 0, 255, 255, "Paused");
        }
        canvas.present();
    }

    pub fn update_inputs(&mut self, event: &Event) {
        self.game_input.update_inputs(event);
    }

    pub fn update(&mut self) {
        if self.game_input.pause_pressed {
            self.game_input.acknowledge_pause();
            if self.game_state.playing() {
                self.paused = ! self.paused;
            }
        }

        if self.game_input.dec_vol_pressed {
            self.game_input.acknowledge_volume_change();
            self.sound_control.decrease_volume(&mut self.animations);
        }
        if self.game_input.inc_vol_pressed {
            self.game_input.acknowledge_volume_change();
            self.sound_control.increase_volume(&mut self.animations);
        }

        if ! self.paused {
            self.frame_count += 1;

            if self.game_state.playing() {
                self.bonus_bomb_collision();
                self.missile_collision();
                self.bomb_collision();
                self.spider_collision();
                if self.ship.waiting_for_changeover() &&
                    self.spiders.clear() && 
                    ! self.bombs.in_flight() && ! self.missile.flying() &&
                    self.ship.enough_delay_for_changeover() {
                    self.ship.proceed_with_changeover();
                }
                if self.letter_bricks.complete() {
                    self.letter_bricks.initiate_expansion();
                }
                if self.letter_bricks.complete() || ! self.ship.life_left() {
                    self.game_state = State::GameOver;
                    self.spiders.turn_sound_off();
                    self.bonus_bomb.turn_sound_off();
                }

                self.missile.update();
                self.ship.update();
                if self.game_input.left_pressed {
                    self.ship.move_left();
                }
                else if self.game_input.right_pressed {
                    self.ship.move_right();
                }
                if self.game_input.fire_pressed {
                    self.ship.launch_missile(&mut self.missile);
                    self.game_input.acknowledge_fire();
                }
            }

            if self.game_state.screen_in_progress() || ! self.game_state.playing() {
                self.base_bricks.update();
                self.letter_bricks.update(self.frame_count);
                self.bonus_bomb.update();
                self.mother.update(&mut self.bonus_bomb, self.frame_count);
                self.bombs.update();
                self.spiders.update(
                    &self.mother,
                    &mut self.base_bricks,
                    &mut self.letter_bricks,
                    &mut self.bombs,
                    (self.ship.in_changeover() && self.game_state.playing()) ||
                        self.ship.protected(),
                    self.frame_count);
            }

            if self.game_state.screen_in_progress() &&
                ! self.spiders.spiders_remain() &&
                ! self.bombs.in_flight() {
                self.screen += 1;
                self.mother.reset();
                self.bonus_bomb.reset();
                self.spiders.reset(self.screen);
                self.frame_count = 0;
                self.screen_start();
            }

            if (! self.game_state.playing()) && self.game_input.start_pressed {
                self.new_game();
            }
            self.animations.update();
            self.background.update();
        }
    }
}
