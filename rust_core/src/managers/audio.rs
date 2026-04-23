use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

pub struct AudioManager {
    pop_sound: Sound,
    spawn_sound: Sound,
    burst_sound: Sound,
}

impl AudioManager {
    pub async fn new() -> Self {
        // Load sounds from the assets directory
        let pop_sound = load_sound_from_bytes(include_bytes!("../../assets/pop.wav"))
            .await
            .unwrap();
        let spawn_sound = load_sound_from_bytes(include_bytes!("../../assets/spawn.wav"))
            .await
            .unwrap();
        let burst_sound = load_sound_from_bytes(include_bytes!("../../assets/burst.wav"))
            .await
            .unwrap();

        Self {
            pop_sound,
            spawn_sound,
            burst_sound,
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
}
