use crate::{api::API, particle::Particle, variant::Variant};

pub struct World {
    pub(crate) particles: Vec<Particle>,
    pub(crate) ambient_heat: u8,
    pub(crate) ambient_pressure: u8,
    pub(crate) ambient_wind: u8,

    pub width: i32,
    pub height: i32,
}

impl Default for World {
    fn default() -> Self {
        Self::new(100, 100)
    }
}

impl World {
    pub fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_idx(x, y);
                let mut particle = World::get_particle(&self, x, y);
                update_particle(particle, API { world: self, x, y });
            }
        }

        fn update_particle(mut particle: Particle, api: API) {
            particle.variant.update(particle, api);
        }
    }

    pub fn new(width: i32, height: i32) -> World {
        let mut particles = Vec::new();
        for _ in 0..width * height {
            particles.push(Particle::new(Variant::Empty, "empty", 0, 0));
        }
        World {
            particles,
            ambient_heat: 0,
            ambient_pressure: 0,
            ambient_wind: 0,
            width,
            height,
        }
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn reset(&mut self) {
        for particle in self.particles.iter_mut() {
            *particle = Particle::new(Variant::Empty, "empty", 0, 0);
        }
    }
}

impl World {
    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn get_particle(&self, x: i32, y: i32) -> Particle {
        let idx = self.get_idx(x, y);
        self.particles[idx]
    }

    pub fn set_particle(&mut self, x: i32, y: i32, variant: Variant) {
        let idx = self.get_idx(x, y);
        self.particles[idx] = Particle::new(variant, "sand", 0, 0);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_set() {
        let world = World::new(100, 100);
        assert_eq!(world.get_particle(0, 0).get_ident(), "empty");
    }
}
