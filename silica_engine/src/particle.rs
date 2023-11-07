use crate::{api::API, variant::Variant};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub variant: Variant,
    pub identifier: &'static str,
    pub ra: u8,
    pub rb: u8,
}

impl Particle {
    pub fn new(variant: Variant, identifier: &'static str, ra: u8, rb: u8) -> Particle {
        Particle {
            variant: variant,
            identifier: identifier,
            ra,
            rb,
        }
    }

    pub fn get_ident(&self) -> &'static str {
        self.identifier
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn update(&mut self, particle: Particle, api: API) {
        // pass
        self.update(*self, api);
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
