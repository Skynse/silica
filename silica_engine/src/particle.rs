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
    };

    res
}
