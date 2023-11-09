use bevy::prelude::Resource;
use silica_engine::{
    variant::Variant,
    variant_type::{variant_type, VariantProperty, VariantType},
};

use crate::gameworld::GameWorld;

#[derive(Resource)]
pub struct Tools {
    pub variant: Variant,
    pub tool_size: usize,
}

impl Tools {
    pub fn paint(&mut self, world: &mut GameWorld, x: usize, y: usize) {
        let half_size = self.tool_size / 2;
        let remainder = if half_size == 0 {
            1
        } else {
            self.tool_size % half_size
        };

        let x1 = if x > half_size { x - half_size } else { 1 };
        let x2 = if x + half_size + remainder < world.width() {
            x + half_size + remainder
        } else {
            world.width() - 1
        };

        let y1 = if y > half_size { y - half_size } else { 1 };
        let y2 = if y + half_size + remainder < world.height() {
            y + half_size + remainder
        } else {
            world.height() - 1
        };

        let radius = (half_size * half_size) as isize;
        for x in x1..x2 {
            for y in y1..y2 {
                if (x as isize - x as isize).pow(2) + (y as isize - y as isize).pow(2) < radius {
                    // first check if the particle is unerasable
                    if variant_type(world.world.get_particle(x as i32, y as i32).variant)
                        .variant_property
                        == VariantProperty::UnErasable
                    {
                        return;
                    }
                    world.world.set_particle(x as i32, y as i32, self.variant);
                }
            }
        }
    }

    pub fn paint_specific_variant(
        &mut self,
        world: &mut GameWorld,
        x: usize,
        y: usize,
        variant: Variant,
    ) {
        let half_size = self.tool_size / 2;
        let remainder = if half_size == 0 {
            1
        } else {
            self.tool_size % half_size
        };

        let x1 = if x > half_size { x - half_size } else { 1 };
        let x2 = if x + half_size + remainder < world.width() {
            x + half_size + remainder
        } else {
            world.width() - 1
        };

        let y1 = if y > half_size { y - half_size } else { 1 };
        let y2 = if y + half_size + remainder < world.height() {
            y + half_size + remainder
        } else {
            world.height() - 1
        };

        let radius = (half_size * half_size) as isize;
        for x in x1..x2 {
            for y in y1..y2 {
                if (x as isize - x as isize).pow(2) + (y as isize - y as isize).pow(2) < radius {
                    world.world.set_particle(x as i32, y as i32, variant);
                }
            }
        }
    }
}
