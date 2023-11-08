use crate::{api::API, particle::Particle};
pub static EMPTY_CELL: Particle = Particle {
    variant: Variant::Empty,
    ra: 0,
    rb: 0,
};
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Variant {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}

impl Variant {
    pub fn update(&self, particle: Particle, api: API) {
        // pass
        match self {
            Variant::Empty => (),
            Variant::Wall => (),
            Variant::Sand => update_sand(particle, api),
        }
    }
}

pub fn update_sand(particle: Particle, mut api: API) {
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);

    // Check the cell below
    if nb.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    }
    // Check and update the cells to the left and right if needed
    else {
        let nbr = api.get(dx + 1, 1);
        let nbl = api.get(dx - 1, 1);

        if nbr.variant == Variant::Empty {
            api.set(dx + 1, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        } else if nbl.variant == Variant::Empty {
            api.set(dx - 1, 1, particle);
            api.set(0, 0, EMPTY_CELL);
        }
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
