use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

pub struct Resources {
    pub font: Font,
    pub pop_sound: Sound,
    pub spawn_sound: Sound,
    pub burst_sound: Sound,
    pub splash_sounds: Vec<Sound>,
}

impl Resources {
    pub async fn load() -> Self {
        Self {
            font: load_ttf_font_from_bytes(include_bytes!("../assets/sniglet_extrabold.ttf"))
                .unwrap(),
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
}
