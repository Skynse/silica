use crate::{api::API, variant::Variant};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub variant: Variant,
    pub ra: u8,
    pub rb: u8,
}

impl Particle {
    pub fn new(variant: Variant, ra: u8, rb: u8) -> Particle {
        Particle {
            variant: variant,
            ra: 100 + rand::thread_rng().gen_range(0..2) * 50 as u8,
            rb,
        }
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn update(&self, api: API) {
        // pass
        self.variant.update(*self, api);
    }
}

pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => (0, 0, 0),
        Variant::Wall => (0x7F, 0x7F, 0x7F),
        Variant::Sand => (0xFF, 0xFF, 0x00),
        Variant::Water => (0x00, 0x00, 0xFF),
    };

    res
}

pub fn interpolate(
    color_1: &(u8, u8, u8),
    color_2: &(u8, u8, u8),
    factor: u8,
    max: u8,
) -> (u8, u8, u8) {
    let factor_f32 = factor as f32 / max as f32;
    let inv_factor_f32 = 1.0 - factor_f32;
    (
        (color_1.0 as f32 * factor_f32 + color_2.0 as f32 * inv_factor_f32) as u8,
        (color_1.1 as f32 * factor_f32 + color_2.1 as f32 * inv_factor_f32) as u8,
        (color_1.2 as f32 * factor_f32 + color_2.2 as f32 * inv_factor_f32) as u8,
    )
}
