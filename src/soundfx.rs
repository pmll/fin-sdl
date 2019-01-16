use std::path::PathBuf;
use animation::{AnimationSeq, Animation, Animations};
use sdl2::mixer::{Chunk, MAX_VOLUME, Channel};

use common;

pub const VOL_STEPS: i32 = 10;
const VOL_STEP: i32 = MAX_VOLUME / VOL_STEPS;

pub struct SoundControl {
    volume: i32,
}

impl SoundControl {
    pub fn new() -> SoundControl {
        let vol = VOL_STEPS / 2;
        let _vol = Channel::all().set_volume(vol * VOL_STEPS);
        SoundControl {volume: vol}
    }

    fn update_channel_volume(&self) {
        let vol = self.volume * VOL_STEP;
        let _vol = Channel::all().set_volume(vol);
    }

    pub fn increase_volume(&mut self, animations: &mut Animations) {
        if self.volume < VOL_STEPS {
            self.volume += 1;
            self.update_channel_volume();
        }
        let animation = Animation::new(AnimationSeq::VolumeChange(self.volume));
        animations.register(animation);
    }

    pub fn decrease_volume(&mut self, animations: &mut Animations) {
        if self.volume > 0 {
            self.volume -= 1;
            self.update_channel_volume();
        }
        let animation = Animation::new(AnimationSeq::VolumeChange(self.volume));
        animations.register(animation);
    }
}

pub struct SoundEffect {
    chunk: Chunk,
}

impl SoundEffect {
    pub fn new(file_name: &str) -> SoundEffect {
        SoundEffect {
            chunk: SoundEffect::create_chunk(
                common::find_asset(&format!("sound/{}", file_name))
            ),
        }
    }

    fn create_chunk(file_path: PathBuf) -> Chunk {
        let sound_chunk_res = Chunk::from_file(&file_path);
        match sound_chunk_res {
            Ok(sound_chunk) => sound_chunk,
            _ => {panic!("Failed to load sound file {:?}", file_path);}
        }
    }

    pub fn play(&self) {
        let _play_res = Channel::all().play(&self.chunk, 0);
    }
}
