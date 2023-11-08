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
    pub fn set(&mut self, dx: i32, dy: i32, particle: particle::Particle) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx >= self.world.width - 1 || ny < 0 || ny >= self.world.height - 1 {
            return;
        }

        let idx = self.world.get_idx(nx, ny);
        self.world.particles[idx] = particle;
    }

    pub fn update_world(&mut self) {
        self.world.tick();
    }

    pub fn reset(&mut self) {
        self.world.reset();
    }

    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..n);
        x
    }

    pub fn swap(&mut self, dx: i32, dy: i32) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx >= self.world.width - 1 || ny < 0 || ny >= self.world.height - 1 {
            return;
        }

        let idx = self.world.get_idx(self.x, self.y);
        let nidx = self.world.get_idx(nx, ny);

        let tmp = self.world.particles[idx];
        self.world.particles[idx] = self.world.particles[nidx];
        self.world.particles[nidx] = tmp;
    }

    pub fn get(&mut self, dx: i32, dy: i32) -> Particle {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx >= self.world.width - 1 || ny < 0 || ny >= self.world.height - 1 {
            return Particle::new(Variant::Empty, 0, 0);
        }
        self.world.get(nx, ny)
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
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        assert_eq!(api.get(0, 0).get_variant(), Variant::Sand);
    }

    #[test]
    fn test_world_set() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        assert_eq!(api.get(0, 0).get_variant(), Variant::Sand);
    }

    #[test]
    fn test_world_reset() {
        let mut world = world::World::new(100, 100);
        let mut api = API {
            world: &mut world,
            x: 0,
            y: 0,
        };
        api.set(0, 0, Particle::new(Variant::Sand, 0, 0));
        api.reset();
        assert_eq!(api.get(0, 0).get_variant(), Variant::Empty);
        assert_eq!(api.get(1, 1).get_variant(), Variant::Empty);
    }
}
