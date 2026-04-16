use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BubbleStyle {
    Standard {
        size: i32,
        max_size: i32,
        color: Color,
    },
    Player,
    OpenAir,
}

impl BubbleStyle {
    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        if *self == BubbleStyle::OpenAir || *other == BubbleStyle::OpenAir {
            return BubbleStyle::OpenAir;
        }

        match (self, other) {
            (
                BubbleStyle::Standard {
                    size: s1,
                    max_size: m1,
                    color,
                },
                BubbleStyle::Standard { size: s2, .. },
            ) => BubbleStyle::Standard {
                size: s1 + s2,
                max_size: *m1,
                color: *color,
            },
            _ => unreachable!("merge should only be called with Standard or OpenAir styles"),
        }
    }
}
