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
        // draw walls with thickness of 2px
        let wall_thickness = 2;
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                if x < wall_thickness
                    || x >= width as i32 - wall_thickness
                    || y < wall_thickness
                    || y >= height as i32 - wall_thickness
                {
                    world.set_particle(x, y, Variant::Wall);
                }
            }
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
