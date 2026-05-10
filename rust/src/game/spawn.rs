use crate::game::state::{GamePhase, SoundEvent, UpdateContext};
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, ChooseRandom};

const AVG_WAIT: f32 = 2.0;
const MAX_BUBBLES: usize = 40;

pub struct SpawnManager {
    pub colors: Vec<Color>,
    pub next_spawn_time: f32,
}

impl SpawnManager {
    pub fn new(colors: Vec<Color>) -> Self {
        let mut manager = Self {
            colors,
            next_spawn_time: 0.0,
        };
        manager.next_spawn_time = Self::get_next_spawn_time();
        manager
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        if ctx.state.graph.bubbles.len() >= MAX_BUBBLES {
            return;
        }

        let count = ctx.state.graph.bubbles.values().count();
        let multiplier = if count < self.colors.len() * 3 {
            5.0
        } else {
            1.0
        };

        self.next_spawn_time -= ctx.dt * multiplier;

        if self.next_spawn_time < 0.0 && ctx.state.phase == GamePhase::Normal {
            self.spawn(&mut ctx.state.graph);
            self.next_spawn_time = Self::get_next_spawn_time();
            ctx.state.sound_events.push(SoundEvent::Spawn);
        }
    }

    fn spawn(&mut self, graph: &mut Graph) {
        let open_air_vertices = graph.get_open_air_vertices();
        if open_air_vertices.is_empty() {
            return;
        }

        let vkey = *open_air_vertices.choose().expect("open air has vertices");

        let adjacent_colors: Vec<Color> = graph.vertices[vkey]
            .edges
            .iter()
            .filter_map(|edge| match graph.bubbles[edge.bubble].style {
                BubbleStyle::Colored { color, .. } => Some(color),
                _ => None,
            })
            .collect();

        let mut valid_colors = self.colors.clone();
        valid_colors.retain(|c| !adjacent_colors.contains(c));

        let color = valid_colors.choose().expect("some colors to choose from");

        graph.spawn(vkey, BubbleStyle::colored(*color));
    }

    fn get_next_spawn_time() -> f32 {
        // Exponential distribution: -log(U) * average
        let u: f32 = gen_range(0.0001, 1.0);
        -u.ln() * AVG_WAIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::GameState;
    use crate::graphics::colors;

    #[test]
    fn test_spawn_manager_speedup() {
        let mut manager = SpawnManager::new(colors::get_group(3));
        let graph = Graph::new(
            BubbleStyle::colored(colors::TURQUOISE),
            BubbleStyle::colored(colors::ROSE),
        );
        let mut state = GameState::new(graph);

        manager.next_spawn_time = 100.0;
        let initial_time = manager.next_spawn_time;
        // 2 colored bubbles, 3 colors -> target is 9. Should speed up by 5.0.
        manager.update(&mut UpdateContext {
            state: &mut state,
            dt: 1.0,
        });

        assert_eq!(manager.next_spawn_time, initial_time - 5.0);
    }

    #[test]
    fn test_spawn_manager_max_bubbles() {
        let mut manager = SpawnManager::new(colors::get_group(3));
        let mut graph = Graph::new(
            BubbleStyle::colored(colors::TURQUOISE),
            BubbleStyle::colored(colors::ROSE),
        );

        // Fill up to MAX_BUBBLES
        while graph.bubbles.len() < MAX_BUBBLES {
            let vkey = graph.vertices.keys().next().unwrap();
            graph.spawn(vkey, BubbleStyle::colored(colors::TURQUOISE));
        }

        let mut state = GameState::new(graph);
        manager.next_spawn_time = 100.0;
        manager.update(&mut UpdateContext {
            state: &mut state,
            dt: 1.0,
        });

        // Should NOT have decremented next_spawn_time because it returns early
        assert_eq!(manager.next_spawn_time, 100.0);
    }
}
