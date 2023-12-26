// animation sequences

use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;
use sdl2::pixels::Color; 

use crate::image::Image;
use crate::text::Text;
use crate::soundfx::VOL_STEPS;

pub enum AnimationSeq {
    ShipExplosion(i32, i32),
    SpiderExplosion(i32, i32),
    BonusBombHit(i32, i32, usize),
    ScreenStart(u32),
    ExtraLife,
    VolumeChange(i32),
}

pub struct Animation {
    animation_seq: AnimationSeq,
    frames_left: u32,
}

impl Animation {
    pub fn new(animation_seq: AnimationSeq) -> Animation {
        let frames_left = match animation_seq {
            AnimationSeq::ShipExplosion(_,_) => 32,
            AnimationSeq::SpiderExplosion(_,_) => 20,
            AnimationSeq::BonusBombHit(_,_,_) => 100,
            AnimationSeq::ScreenStart(_) => 100,
            AnimationSeq::ExtraLife => 50,
            AnimationSeq::VolumeChange(_) => 100,
        };
        Animation {
            animation_seq,
            frames_left
        }
    }

    pub fn finished(&self) -> bool {
        self.frames_left == 0
    }
}

pub struct Animations<'a, 'b> {
    animation: Vec<Animation>,
    ship_explosion_image: [Image<'a>; 4],
    spider_explosion_image: [Image<'a>; 4],
    bonus_bomb_image: [Image<'a>; 3],
    text40: Text<'a, 'b>,
}

impl<'a, 'b> Animations<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl2TtfContext) -> Animations<'a, 'b> {
        Animations {
            animation: Vec::new(),
            ship_explosion_image: [
                Image::new(texture_creator, "ship_explosion1.png", 30, 40),
                Image::new(texture_creator, "ship_explosion2.png", 30, 40),
                Image::new(texture_creator, "ship_explosion3.png", 30, 40),
                Image::new(texture_creator, "ship_explosion4.png", 30, 40),
            ],
            spider_explosion_image: [
                Image::new(texture_creator, "spider_explosion1.png", 30, 40),
                Image::new(texture_creator, "spider_explosion2.png", 30, 40),
                Image::new(texture_creator, "spider_explosion3.png", 30, 40),
                Image::new(texture_creator, "spider_explosion4.png", 30, 40),
           ],
           bonus_bomb_image: [
                Image::new(texture_creator, "bonus10_1.png", 50, 22),
                Image::new(texture_creator, "bonus30_1.png", 50, 22),
                Image::new(texture_creator, "bonus50_1.png", 50, 22),
           ],
           text40: Text::new(ttf_context, 40),
        }
    }

    pub fn unregister_finished(&mut self) {
        self.animation.retain(|a| ! a.finished());
    }

    pub fn register(&mut self, animation: Animation) {
        // if we get a volume change through, it replaces any ongoing
        // animation for it
        if let AnimationSeq::VolumeChange(_) = animation.animation_seq {
            self.animation.retain(|a|
                if let AnimationSeq::VolumeChange(_) = a.animation_seq
                    {false}
                else
                    {true});
        }
        self.animation.push(animation);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for a in &self.animation {
            match a.animation_seq {
                AnimationSeq::ShipExplosion(x, y) => {
                    self.render_ship_explosion(canvas, x, y, a.frames_left);
                }
                AnimationSeq::SpiderExplosion(x, y) => {
                    self.render_spider_explosion(canvas, x, y, a.frames_left);
                }
                AnimationSeq::BonusBombHit(x, y, bomb_type) => {
                    self.render_bomb_hit(canvas, x, y, bomb_type, a.frames_left);
                }
                AnimationSeq::ScreenStart(num) => {
                    self.render_screen_start(canvas, num, a.frames_left);
                }
                AnimationSeq::ExtraLife => {
                    self.render_extra_life(canvas, a.frames_left);
                }
                AnimationSeq::VolumeChange(new_vol) => {
                    self.render_volume(canvas, new_vol, a.frames_left);
                }
            }
        }
    }

    pub fn update(&mut self) {
        for a in &mut self.animation.iter_mut() {
             a.frames_left -= 1;
        }
        self.unregister_finished();
    }    

    fn render_ship_explosion(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, frames_left: u32) {
        self.ship_explosion_image[((32 - frames_left) / 8) as usize].render(canvas, x, y);
    }

    fn render_spider_explosion(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, frames_left: u32) {
        self.spider_explosion_image[((20 - frames_left) / 5) as usize].render(canvas, x, y);
    }

    fn render_bomb_hit(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, bomb_type: usize, frames_left: u32) {
        let scale = frames_left as f64 / 100.0;
        let angle = scale * 1800.0;
        let lx = x - (25.0 * scale) as i32;
        let ly = y - (11.0 * scale) as i32;
        self.bonus_bomb_image[bomb_type].render_angle(canvas, lx, ly, angle, scale);
    }

    fn render_screen_start(&self, canvas: &mut Canvas<Window>, num: u32, frames_left: u32) {
        self.text40.render(canvas, 120, 350, 79, 120, 181,
           ((frames_left * 255 / 100)) as u8,
           &format!("Get ready for attack {}", num));
    }

    fn render_extra_life(&self, canvas: &mut Canvas<Window>, frames_left: u32) {
        self.text40.render(canvas, 210, 420, 255, 0, 0, (5 * frames_left) as u8,
            "Extra Life!");
    }

    fn render_volume(&self, canvas: &mut Canvas<Window>, volume: i32, frames_left: u32) {
        canvas.set_draw_color(Color::RGBA(0, 0, 255, (frames_left * 255 / 100) as u8));
        let rect = Rect::new(245, 240, 10 * (VOL_STEPS as u32 + 1), 2);
        canvas.fill_rect(rect).unwrap();
        for i in 1..volume + 1 {
            let bar_x = 245 + i * 10;
            let bar_h = 30 * i / VOL_STEPS;
            let rect = Rect::new(bar_x, 238 - bar_h, 8, bar_h as u32);
            canvas.fill_rect(rect).unwrap();
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
}
