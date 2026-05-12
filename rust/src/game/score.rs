use crate::game::state::{ScoreEvent, UpdateContext};
use crate::graphics::{colors, RenderContext};
use macroquad::prelude::*;

const CHAIN_RESET_TIME: f32 = 2.0;
const POINT_DISPLAY_TIME: f32 = 1.0;
const POINT_MAX_HEIGHT: f32 = 150.0;
const WALL_BURST_POINTS: i64 = 10;

#[derive(Default)]
struct ChainTimer {
    timer: f32,
    count: i32,
}

impl ChainTimer {
    fn re_up(&mut self) {
        if self.timer > 0.0 {
            self.count += 1;
        } else {
            self.count = 1;
        }
        self.timer = CHAIN_RESET_TIME;
    }

    fn update(&mut self, dt: f32) {
        self.timer -= dt;
    }

    fn get_count(&self) -> i32 {
        if self.timer > 0.0 {
            self.count
        } else {
            0
        }
    }
}

struct RisingPoint {
    text: String,
    position: Vec2,
    timer: f32,
}

#[derive(Default)]
pub struct ScoreManager {
    burst_chain: ChainTimer,
    pop_chain: ChainTimer,
    rising_points: Vec<RisingPoint>,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        self.burst_chain.update(ctx.dt);
        self.pop_chain.update(ctx.dt);

        let mut total_points = 0;
        let mut new_rising_points = Vec::new();

        for event in ctx.state.score_events.drain(..) {
            match event {
                ScoreEvent::Burst { position } => {
                    self.burst_chain.re_up();
                    let points = WALL_BURST_POINTS * self.burst_chain.get_count() as i64;
                    total_points += points;
                    new_rising_points.push(RisingPoint {
                        text: format!("{points}"),
                        position,
                        timer: 0.0,
                    });
                }
                ScoreEvent::Pop { position, size } => {
                    self.pop_chain.re_up();
                    let points =
                        WALL_BURST_POINTS * size as i64 * self.pop_chain.get_count() as i64;
                    total_points += points;
                    new_rising_points.push(RisingPoint {
                        text: format!("{points}"),
                        position,
                        timer: 0.0,
                    });
                }
            }
        }

        if total_points > 0 {
            ctx.state.keeper.add_points(total_points);
            self.rising_points.extend(new_rising_points);
        }

        self.rising_points.retain_mut(|rp| {
            rp.timer += ctx.dt;
            rp.timer < POINT_DISPLAY_TIME
        });
    }

    pub fn draw(&self, ctx: &RenderContext) {
        // 1. Draw Rising Points (in World Space)
        if !self.rising_points.is_empty() {
            set_camera(ctx.camera);
            for rp in &self.rising_points {
                let rise = (rp.timer / POINT_DISPLAY_TIME) * POINT_MAX_HEIGHT;
                let text_dims = measure_text(&rp.text, Some(ctx.font), 32, 1.0);

                // Draw text with shadow
                let draw_pos = vec2(rp.position.x - text_dims.width / 2.0, rp.position.y - rise);

                let shadow_pos = draw_pos + 2.0;
                draw_text_ex(
                    &rp.text,
                    shadow_pos.x,
                    shadow_pos.y,
                    TextParams {
                        font: Some(ctx.font),
                        font_size: 32,
                        color: colors::BLACK,
                        ..Default::default()
                    },
                );
                draw_text_ex(
                    &rp.text,
                    draw_pos.x,
                    draw_pos.y,
                    TextParams {
                        font: Some(ctx.font),
                        font_size: 32,
                        color: colors::WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        // 2. Draw Total Score and Chains (in Screen Space)
        set_default_camera();

        let score_text = format!("{score}", score = ctx.state.keeper.score);
        let score_dims = measure_text(&score_text, Some(ctx.font), 32, 1.0);

        // Draw score at the top right
        draw_text_ex(
            &score_text,
            (screen_width() - score_dims.width - 20.0).floor(),
            40.0,
            TextParams {
                font: Some(ctx.font),
                font_size: 32,
                color: colors::WHITE,
                ..Default::default()
            },
        );

        let pop_chain_count = self.pop_chain.get_count();
        let burst_chain_count = self.burst_chain.get_count();

        if pop_chain_count > 1 {
            let chain_text = format!("{pop_chain_count} Scatti Concatenati!");
            let chain_dims = measure_text(&chain_text, Some(ctx.font), 32, 1.0);
            draw_text_ex(
                &chain_text,
                (screen_width() / 2.0 - chain_dims.width / 2.0).floor(),
                80.0,
                TextParams {
                    font: Some(ctx.font),
                    font_size: 32,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
        } else if burst_chain_count > 1 {
            let chain_text = format!("{burst_chain_count} Concatenati!");
            let chain_dims = measure_text(&chain_text, Some(ctx.font), 32, 1.0);
            draw_text_ex(
                &chain_text,
                (screen_width() / 2.0 - chain_dims.width / 2.0).floor(),
                80.0,
                TextParams {
                    font: Some(ctx.font),
                    font_size: 32,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
