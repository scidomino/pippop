use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};
use macroquad::prelude::*;
use macroquad::rand;

pub struct Resources {
    pub font: Font,
    pub pop_sound: Sound,
    pub spawn_sound: Sound,
    pub burst_sound: Sound,
    pub splash_sounds: Vec<Sound>,
}

impl Resources {
    pub async fn load() -> Self {
        let mut font =
            load_ttf_font_from_bytes(include_bytes!("../assets/sniglet_extrabold.ttf")).unwrap();
        font.set_filter(FilterMode::Nearest);
        Self {
            font,
            pop_sound: load_sound_from_bytes(include_bytes!("../assets/pop.wav"))
                .await
                .unwrap(),
            spawn_sound: load_sound_from_bytes(include_bytes!("../assets/spawn.wav"))
                .await
                .unwrap(),
            burst_sound: load_sound_from_bytes(include_bytes!("../assets/burst.wav"))
                .await
                .unwrap(),
            splash_sounds: vec![
                load_sound_from_bytes(include_bytes!("../assets/splash1.wav"))
                    .await
                    .unwrap(),
                load_sound_from_bytes(include_bytes!("../assets/splash2.wav"))
                    .await
                    .unwrap(),
                load_sound_from_bytes(include_bytes!("../assets/splash3.wav"))
                    .await
                    .unwrap(),
                load_sound_from_bytes(include_bytes!("../assets/splash4.wav"))
                    .await
                    .unwrap(),
                load_sound_from_bytes(include_bytes!("../assets/splash5.wav"))
                    .await
                    .unwrap(),
                load_sound_from_bytes(include_bytes!("../assets/splash6.wav"))
                    .await
                    .unwrap(),
            ],
        }
    }

    pub fn play_pop(&self) {
        play_sound_once(&self.pop_sound);
    }

    pub fn play_spawn(&self) {
        play_sound_once(&self.spawn_sound);
    }

    pub fn play_burst(&self) {
        play_sound_once(&self.burst_sound);
    }

    pub fn play_swap(&self) {
        if !self.splash_sounds.is_empty() {
            let idx = rand::gen_range(0, self.splash_sounds.len());
            play_sound_once(&self.splash_sounds[idx]);
        }
    }
}
