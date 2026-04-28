use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

pub struct Resources {
    pub font: Font,
    pub pop_sound: Sound,
    pub spawn_sound: Sound,
    pub burst_sound: Sound,
}

impl Resources {
    pub async fn load() -> Self {
        let font_bytes = include_bytes!("../assets/sniglet_extrabold.ttf");
        let font = load_ttf_font_from_bytes(font_bytes).unwrap();

        let pop_sound = load_sound_from_bytes(include_bytes!("../assets/pop.wav"))
            .await
            .unwrap();
        let spawn_sound = load_sound_from_bytes(include_bytes!("../assets/spawn.wav"))
            .await
            .unwrap();
        let burst_sound = load_sound_from_bytes(include_bytes!("../assets/burst.wav"))
            .await
            .unwrap();

        Self {
            font,
            pop_sound,
            spawn_sound,
            burst_sound,
        }
    }
}
