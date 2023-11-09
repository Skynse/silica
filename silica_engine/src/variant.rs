use crate::{
    api::API,
    particle::Particle,
    variant_type::{self, variant_type, VariantProperty, VARIANTS},
};
pub static EMPTY_CELL: Particle = Particle {
    variant: Variant::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Fire = 4,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Variant::Empty => "Empty",
            Variant::Wall => "Wall",
            Variant::Sand => "Sand",
            Variant::Water => "Water",
            Variant::Fire => "Fire",
        };

        write!(f, "{}", res)
    }
}

impl Variant {
    pub fn update(&self, particle: Particle, api: API) {
        // pass
        match self {
            Variant::Empty => (),
            Variant::Wall => (),
            Variant::Sand => update_sand(particle, api),
            Variant::Water => update_water(particle, api),
            Variant::Fire => update_fire(particle, api),
        }
    }
}

pub fn update_sand(particle: Particle, mut api: API) {
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);

    if nb.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.variant == Variant::Empty {
        api.set(dx + 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.variant == Variant::Empty {
        api.set(dx - 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    }
}

pub fn update_fire(particle: Particle, mut api: API) {
    let dx = api.rand_dir();
    let nb = api.get(dx, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);
    let nl = api.get(-1, 0);
    let nr = api.get(1, 0);

    // check if neighbor is powder property and displace if heavier
    let nb_type = variant_type(nb.variant);

    if nb_type.variant_property == VariantProperty::Powder {
        if nb_type.weight > VARIANTS[particle.variant as usize].weight {
            api.set(0, 1, particle);
            api.set(0, 0, nb);
        }
    }

    if nb.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.variant == Variant::Empty {
        api.set(dx + 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.variant == Variant::Empty {
        api.set(dx - 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nl.variant == Variant::Empty {
        api.set(-1, 0, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nr.variant == Variant::Empty {
        api.set(1, 0, particle);
        api.set(0, 0, EMPTY_CELL);
    }
}

pub fn update_water(particle: Particle, mut api: API) {
    let dx = api.rand_dir();
    let nb = api.get(0, 1);
    let nbr = api.get(dx + 1, 1);
    let nbl = api.get(dx - 1, 1);
    let nl = api.get(-1, 0);
    let nr = api.get(1, 0);

    // check if neighbor is powder property and displace if heavier
    let nb_type = variant_type(nb.variant);

    if nb_type.variant_property == VariantProperty::Powder {
        if nb_type.weight > VARIANTS[particle.variant as usize].weight {
            api.set(0, 1, particle);
            api.set(0, 0, nb);
        }
    }

    if nb.variant == Variant::Empty {
        api.set(dx, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbr.variant == Variant::Empty {
        api.set(dx + 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nbl.variant == Variant::Empty {
        api.set(dx - 1, 1, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nl.variant == Variant::Empty {
        api.set(-1, 0, particle);
        api.set(0, 0, EMPTY_CELL);
    } else if nr.variant == Variant::Empty {
        api.set(1, 0, particle);
        api.set(0, 0, EMPTY_CELL);
    }
}
pub fn particle_to_color(variant: Variant) -> (u8, u8, u8) {
    let res = match variant {
        Variant::Empty => VARIANTS[0].color,
        Variant::Wall => VARIANTS[1].color,
        Variant::Sand => VARIANTS[2].color,
        Variant::Water => VARIANTS[3].color,
        Variant::Fire => VARIANTS[4].color,
    };

    res
}
