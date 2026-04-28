use crate::resources::Resources;
use macroquad::audio::{play_sound_once, Sound};

pub struct AudioManager {
    pop_sound: Sound,
    spawn_sound: Sound,
    burst_sound: Sound,
}

impl AudioManager {
    pub fn new(resources: &Resources) -> Self {
        Self {
            pop_sound: resources.pop_sound.clone(),
            spawn_sound: resources.spawn_sound.clone(),
            burst_sound: resources.burst_sound.clone(),
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
