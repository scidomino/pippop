use crate::game::state::{GamePhase, SoundEvent, UpdateContext};
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams};
use macroquad::rand::ChooseRandom;

#[derive(Default)]
pub struct SoundManager;

impl SoundManager {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, ctx: &mut UpdateContext) {
        if matches!(ctx.state.phase, GamePhase::Paused(_)) {
            return;
        }
        if let Some(resources) = ctx.resources {
            for event in ctx.state.sound_events.drain(..) {
                match event {
                    SoundEvent::Pop => play_sound_once(&resources.pop_sound),
                    SoundEvent::Spawn => play_sound_once(&resources.spawn_sound),
                    SoundEvent::Burst => play_sound_once(&resources.burst_sound),
                    SoundEvent::Swap => {
                        play_sound(
                            resources
                                .splash_sounds
                                .choose()
                                .expect("has at least one splash noise"),
                            PlaySoundParams {
                                looped: false,
                                volume: 0.4,
                            },
                        );
                    }
                }
            }
        } else {
            ctx.state.sound_events.clear();
        }
    }
}
