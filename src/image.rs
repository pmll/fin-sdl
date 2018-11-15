use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use common::find_asset;

pub struct Image<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> Image<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>,
        file_name: &str, width: u32, height: u32) -> Image<'a> {
        let image_path = find_asset(&format!("image/{}", file_name));
        Image {
            texture: texture_creator.load_texture(image_path).unwrap(),
            width,
            height,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        let rect = Rect::new(x, y, self.width, self.height);
        canvas.copy(&self.texture, None, rect).unwrap();
    }

    pub fn render_resize(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, scale: f64) {
        let rect = Rect::new(x, y, (self.width as f64 * scale) as u32,
            (self.height as f64 * scale) as u32);
        canvas.copy(&self.texture, None, rect).unwrap();
    }

    pub fn render_angle(&self, canvas: &mut Canvas<Window>, x: i32, y: i32,
        angle: f64, scale: f64) {
        let rect = Rect::new(x, y, (self.width as f64 * scale) as u32,
            (self.height as f64 * scale) as u32);
        canvas.copy_ex(&self.texture, None, rect, angle, None, false, false).unwrap();
    }
}

