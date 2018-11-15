use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use common;

pub struct Text<'a, 'b> {
  font: Font<'a, 'b>,
}

impl<'a, 'b> Text<'a, 'b> {
    pub fn new(ttf_context: &'a Sdl2TtfContext, size: u16) -> Text<'a, 'b> {
        let font_path = common::find_asset("font/FiraSans-Regular.ttf");
        let font = ttf_context.load_font(font_path, size).unwrap();
        Text {font}
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, 
       r: u8, g: u8, b: u8, a: u8, str: &str) {
        let texture_creator = canvas.texture_creator();
        // fixme: alpha channel doesn't work
        let surface = self.font.render(str).blended(Color::RGBA(r, g, b, a)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let rect = Rect::new(x, y, width, height);
        canvas.copy(&texture, None, rect).unwrap();
    }
}
