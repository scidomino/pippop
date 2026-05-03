use crate::game::state::{SoundEvent, UpdateContext};
use crate::resources::Resources;
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams};
use macroquad::rand;

pub struct SoundManager<'a> {
    resources: &'a Resources,
}

impl<'a> SoundManager<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        Self { resources }
    }

    pub fn play_pop(&self) {
        play_sound_once(&self.resources.pop_sound);
    }

    pub fn play_spawn(&self) {
        play_sound_once(&self.resources.spawn_sound);
    }

    pub fn play_burst(&self) {
        play_sound_once(&self.resources.burst_sound);
    }

    pub fn play_swap(&self) {
        if !self.resources.splash_sounds.is_empty() {
            let idx = rand::gen_range(0, self.resources.splash_sounds.len());
            play_sound(
                &self.resources.splash_sounds[idx],
                PlaySoundParams {
                    looped: false,
                    volume: 0.4,
                },
            );
        }
    }

    pub fn update(&self, ctx: &mut UpdateContext) {
        for event in ctx.state.sound_events.drain(..) {
            match event {
                SoundEvent::Pop => self.play_pop(),
                SoundEvent::Spawn => self.play_spawn(),
                SoundEvent::Burst => self.play_burst(),
                SoundEvent::Swap => self.play_swap(),
            }
        }
    }
}
