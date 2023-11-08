use bevy::prelude::Component;
use silica_engine::{variant::Variant, world::World};

#[derive(Component)]
pub struct GameWorld {
    pub world: World,
    width: usize,
    height: usize,
    pub running: bool,
}

impl GameWorld {
    pub fn new(width: f32, height: f32) -> GameWorld {
        let mut world = World::new(width as i32, height as i32);
        // draw walls
        for x in 0..width as usize - 1 {
            world.set_particle(x as i32, 0, Variant::Wall);
            world.set_particle(x as i32, height as i32 - 1, Variant::Wall);
        }
        for y in 0..height as usize - 1 {
            world.set_particle(0, y as i32, Variant::Wall);
            world.set_particle(width as i32 - 1, y as i32, Variant::Wall);
        }

        GameWorld {
            world,
            width: width as usize,
            height: height as usize,
            running: true,
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
