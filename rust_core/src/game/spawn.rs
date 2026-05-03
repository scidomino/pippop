use crate::game::state::GamePhase;
use crate::game::state::{GameEvent, GameState};
use crate::graph::bubble::BubbleStyle;
use crate::graph::vertex::VertexKey;
use crate::graph::Graph;
use crate::graphics::RenderContext;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, ChooseRandom};

const AVG_WAIT: f32 = 2.0;
const MAX_BUBBLES: usize = 40;

pub struct SpawnManager {
    pub colors: Vec<Color>,
    pub next_spawn_time: f32,
    pub next_color: Color,
}

impl SpawnManager {
    pub fn new(colors: Vec<Color>) -> Self {
        let next_color = *colors.choose().expect("colors is non-empty");
        let mut manager = Self {
            colors,
            next_spawn_time: 0.0,
            next_color,
        };
        manager.next_spawn_time = manager.get_next_spawn_time();
        manager
    }

    pub fn update(&mut self, state: &mut GameState, dt: f32) {
        if state.graph.bubbles.len() >= MAX_BUBBLES {
            return;
        }

        let count = state.graph.bubbles.values().count();
        let multiplier = if count < self.colors.len() * 3 {
            5.0
        } else {
            1.0
        };

        self.next_spawn_time -= dt * multiplier;

        if self.next_spawn_time < 0.0 && state.phase == GamePhase::Normal {
            self.spawn(&mut state.graph);
            self.next_spawn_time = self.get_next_spawn_time();
            self.next_color = *self.colors.choose().expect("colors is non-empty");
            state.events.push(GameEvent::Spawn);
        }
    }

    pub fn draw(&self, _ctx: &RenderContext) {
        set_default_camera();

        let radius = 20.0 * 2.0f32.powf(-self.next_spawn_time.max(0.0));
        let x = screen_width() - 30.0;
        let y = 30.0;
        draw_circle(x, y, radius, self.next_color);
        draw_circle_lines(x, y, radius, 2.0, WHITE);
    }

    fn spawn(&mut self, graph: &mut Graph) {
        let open_air_vertices = graph.get_open_air_vertices();

        if open_air_vertices.is_empty() {
            return;
        }

        let color = self.next_color;

        let vkey = *open_air_vertices
            .iter()
            .filter(|&&vkey| !self.vertex_borders_color(graph, vkey, color))
            .copied()
            .collect::<Vec<VertexKey>>()
            .choose()
            .unwrap_or_else(|| open_air_vertices.choose().expect("open air has vertices"));

        graph.spawn(vkey, BubbleStyle::colored(color));
    }

    fn vertex_borders_color(&self, graph: &Graph, vkey: VertexKey, target_color: Color) -> bool {
        (&graph.vertices[vkey])
            .edges
            .iter()
            .filter_map(|e| graph.bubbles.get(e.bubble))
            .any(|bubble| match bubble.style {
                BubbleStyle::Colored { color, .. } => color == target_color,
                _ => false,
            })
    }

    fn get_next_spawn_time(&self) -> f32 {
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
        manager.update(&mut state, 1.0);

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
        manager.update(&mut state, 1.0);

        // Should NOT have decremented next_spawn_time because it returns early
        assert_eq!(manager.next_spawn_time, 100.0);
    }
}
