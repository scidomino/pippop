use crate::game::state::{SoundEvent, UpdateContext};
use crate::resources::Resources;

pub struct SoundManager<'a> {
    resources: &'a Resources,
}

impl<'a> SoundManager<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        Self { resources }
    }

    pub fn update(&self, ctx: &mut UpdateContext) {
        for event in ctx.state.sound_events.drain(..) {
            match event {
                SoundEvent::Pop => self.resources.play_pop(),
                SoundEvent::Spawn => self.resources.play_spawn(),
                SoundEvent::Burst => self.resources.play_burst(),
                SoundEvent::Swap => self.resources.play_swap(),
            }
        }
    }
}
