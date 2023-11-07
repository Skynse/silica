use rand::Rng;

use crate::{
    particle::{self, Particle},
    variant::Variant,
    world,
};

pub struct API<'a> {
    pub(crate) world: &'a mut world::World,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl<'a> API<'a> {
    pub fn set(&mut self, x: i32, y: i32, particle: particle::Particle) {
        let idx = (y * self.world.width + x) as usize;
        self.world.particles[idx] = particle;
    }

    pub fn update_world(&mut self) {
        self.world.tick();
    }

    pub fn reset(&mut self) {
        self.world.reset();
    }

    pub fn rand_dir(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1..1);
        x
    }

    pub fn get(&mut self, dx: i32, dy: i32) -> Particle {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx >= self.world.width - 1 || ny < 0 || ny >= self.world.height - 1 {
            return Particle::new(Variant::Empty, "empty", 0, 0);
        }
        self.world.get_particle(nx, ny)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use crate::world::World;

    use super::*;

    #[test]
    fn test_api() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, "sand", 0, 0));
        assert_eq!(api.get(0, 0).get_ident(), "sand");
    }

    #[test]
    fn test_world_set() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, "sand", 0, 0));
        assert_eq!(api.get(0, 0).get_ident(), "sand");
    }

    #[test]
    fn test_world_reset() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, "sand", 0, 0));
        api.reset();
        assert_eq!(api.get(0, 0).get_ident(), "empty");
        assert_eq!(api.get(1, 1).get_ident(), "empty");
    }
}
