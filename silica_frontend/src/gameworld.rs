use bevy::prelude::Component;
use silica_engine::{variant::Variant, world::World};

#[derive(Component)]
pub struct GameWorld {
    pub world: World,
    width: usize,
    height: usize,
}

impl GameWorld {
    pub fn new(width: f32, height: f32) -> GameWorld {
        let mut world = World::new(width as i32, height as i32);
        for x in 0..world.width {
            world.set_particle(x, 0, Variant::Wall);
            world.set_particle(x, world.height - 1, Variant::Wall);
        }

        for y in 0..world.height {
            world.set_particle(0, y, Variant::Wall);
            world.set_particle(world.width - 1, y, Variant::Wall);
        }

        GameWorld {
            world,
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn tick(&mut self) {
        self.world.tick();
    }
}
