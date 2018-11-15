use std::path::PathBuf;
use find_folder;

pub const SCREEN_WIDTH: u32 = 600;
pub const SCREEN_HEIGHT: u32 = 700;
pub const UPDATE_FPS: u32 = 60;

#[derive(Copy, Clone)]
pub struct TargetBrick {
    pub x: i32,
    pub y: i32,
    pub brick_id: usize,
}

pub struct ScreenObjectArea {
    tl_x: i32,
    tl_y: i32,
    br_x: i32,
    br_y: i32,
}

impl ScreenObjectArea {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> ScreenObjectArea {
        ScreenObjectArea {tl_x: x, tl_y: y, br_x: x + w as i32, br_y: y + h as i32}
    }

    pub fn collides(&self, col_area: ScreenObjectArea) -> bool {
        col_area.br_x > self.tl_x && col_area.tl_x < self.br_x &&
        col_area.br_y > self.tl_y && col_area.tl_y < self.br_y
    }
}

pub fn find_asset(file_name: &str) -> PathBuf {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    assets.join(file_name)
}

