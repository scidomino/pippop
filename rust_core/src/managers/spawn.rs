use crate::graph::bubble::BubbleStyle;
use crate::graph::vertex::VertexKey;
use crate::graph::Graph;
use macroquad::prelude::Color;
use macroquad::rand::{gen_range, ChooseRandom};

pub trait SpawnTimer {
    fn get_average_wait(&self, bubble_count: usize, total_play_time: f32) -> f32;
}

pub struct RatchetSpawnTimer {
    pub starting_wait: f32,
    pub doubling_time: f32,
}

impl SpawnTimer for RatchetSpawnTimer {
    fn get_average_wait(&self, _bubble_count: usize, total_play_time: f32) -> f32 {
        self.starting_wait * 0.5f32.powf(total_play_time / self.doubling_time)
    }
}

pub struct SpawnManager {
    pub colors: Vec<Color>,
    pub spawn_timer: Box<dyn SpawnTimer>,
    pub next_spawn_time: f32,
    pub total_play_time: f32,
}
impl SpawnManager {
    pub fn new(colors: Vec<Color>, spawn_timer: Box<dyn SpawnTimer>) -> Self {
        let mut manager = Self {
            colors,
            spawn_timer,
            next_spawn_time: 0.0,
            total_play_time: 0.0,
        };
        manager.next_spawn_time = manager.get_next_spawn_time(3);
        manager
    }

    pub fn update(&mut self, dt: f32) {
        self.total_play_time += dt;
        self.next_spawn_time -= dt;
    }

    pub fn possibly_spawn(&mut self, graph: &mut Graph) {
        if self.next_spawn_time < 0.0 {
            self.spawn(graph);
            self.next_spawn_time = self.get_next_spawn_time(graph.bubbles.len());
        }
    }

    fn spawn(&mut self, graph: &mut Graph) {
        let open_air_vertices = graph.get_open_air_vertices();

        if open_air_vertices.is_empty() {
            return;
        }

        let vkey = *open_air_vertices.choose().unwrap();
        let choices = self.get_distant_colors(graph, vkey);

        let color = if choices.is_empty() {
            *self.colors.choose().unwrap()
        } else {
            *choices.choose().unwrap()
        };

        graph.spawn(
            vkey,
            BubbleStyle::Standard {
                size: 1,
                max_size: 5,
                color,
            },
        );
    }

    fn get_distant_colors(&self, graph: &Graph, vkey: VertexKey) -> Vec<Color> {
        let vertex = &graph.vertices[vkey];

        let nearby_colors: Vec<_> = vertex
            .edges
            .iter()
            .flat_map(|e| [e.bubble, graph.get_edge(e.twin).bubble])
            .filter_map(|bkey| match graph.bubbles.get(bkey)?.style {
                BubbleStyle::Standard { color, .. } => Some(color),
                _ => None,
            })
            .collect();

        self.colors
            .iter()
            .filter(|c| !nearby_colors.contains(c))
            .cloned()
            .collect()
    }

    fn get_next_spawn_time(&self, bubble_count: usize) -> f32 {
        let average_wait = self
            .spawn_timer
            .get_average_wait(bubble_count, self.total_play_time);
        // Exponential distribution: -log(U) * average
        let u: f32 = gen_range(0.0001, 1.0);
        -u.ln() * average_wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::colors;

    #[test]
    fn test_ratchet_timer_progression() {
        let timer = RatchetSpawnTimer {
            starting_wait: 10.0,
            doubling_time: 100.0,
        };

        // At t=0, wait should be 10
        assert_eq!(timer.get_average_wait(0, 0.0), 10.0);
        // At t=100 (one doubling time), wait should be 5
        assert_eq!(timer.get_average_wait(0, 100.0), 5.0);
        // At t=200, wait should be 2.5
        assert_eq!(timer.get_average_wait(0, 200.0), 2.5);
    }

    #[test]
    fn test_spawn_manager_update() {
        let timer = Box::new(RatchetSpawnTimer {
            starting_wait: 10.0,
            doubling_time: 100.0,
        });
        let mut manager = SpawnManager::new(colors::get_group(3), timer);

        let initial_time = manager.next_spawn_time;
        manager.update(1.0);

        assert_eq!(manager.total_play_time, 1.0);
        assert_eq!(manager.next_spawn_time, initial_time - 1.0);
    }

    #[test]
    fn test_spawn_integration() {
        let mut graph = Graph::new();
        graph.init(
            BubbleStyle::Standard {
                size: 1,
                max_size: 5,
                color: colors::TURQUOISE,
            },
            BubbleStyle::Standard {
                size: 1,
                max_size: 5,
                color: colors::ROSE,
            },
        );
        let initial_bubbles = graph.bubbles.len();

        let timer = Box::new(RatchetSpawnTimer {
            starting_wait: 1.0,
            doubling_time: 100.0,
        });
        let mut manager = SpawnManager::new(colors::get_group(3), timer);

        // Force a spawn by setting time to negative
        manager.next_spawn_time = -1.0;
        manager.possibly_spawn(&mut graph);

        assert_eq!(graph.bubbles.len(), initial_bubbles + 1);
        assert!(manager.next_spawn_time > 0.0);
    }
}
