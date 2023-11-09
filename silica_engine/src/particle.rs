use crate::{api::API, variant::Variant, variant_type::VARIANTS};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub variant: Variant,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
}

impl Particle {
    pub fn new(variant: Variant, ra: u8, rb: u8) -> Particle {
        Particle {
            variant: variant,
            ra: 100 + rand::thread_rng().gen_range(0..2) * 50 as u8,
            rb,
            clock: 0,
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
        Variant::Wall => VARIANTS[1].color,
        Variant::Sand => VARIANTS[2].color,
        Variant::Water => VARIANTS[3].color,
        Variant::Fire => VARIANTS[4].color,
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
