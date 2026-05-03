use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

macro_rules! load_sound {
    ($path:expr) => {
        load_sound_from_bytes(include_bytes!($path)).await.unwrap()
    };
}

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
            pop_sound: load_sound!("../assets/pop.wav"),
            spawn_sound: load_sound!("../assets/spawn.wav"),
            burst_sound: load_sound!("../assets/burst.wav"),
            splash_sounds: vec![
                load_sound!("../assets/splash1.wav"),
                load_sound!("../assets/splash2.wav"),
                load_sound!("../assets/splash3.wav"),
                load_sound!("../assets/splash4.wav"),
                load_sound!("../assets/splash5.wav"),
                load_sound!("../assets/splash6.wav"),
            ],
        }
    }
}
