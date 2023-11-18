mod manager;
mod utils;

use macroquad::miniquad::window::{screen_size, show_keyboard};
use macroquad::prelude::*;

use ::rand::Rng;
use manager::{GameProperties, Property, Tool, WorldInfo};
use rayon::prelude::*;
use silica_engine::group::ElementManager;
use silica_engine::{prelude::*, variant, world};
use utils::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Silica".to_owned(),
        window_width: 1390,
        window_height: 900,
        fullscreen: false,

        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initialization

    let w: usize = 311;
    let h: usize = 183;
    let mut image = Image::gen_image_color(w as u16, h as u16, color_u8!(13, 16, 20, 1));
    let mut world: World = World::new(w as i32, h as i32);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut game_properties = GameProperties {
        tool_radius: 10.0,
        tool_type: Tool::ElementTool(Variant::Sand),
        selected_group_idx: 0,
        hovering_over: EMPTY_CELL,
        hovering_temperature: 0.0,
        left_mouse_down: false,
        right_mouse_down: false,
    };
    let mut world_info = WorldInfo {
        fps: 0,
        properties: game_properties,
        world_width: w,
        world_height: h,
    };

    draw_walls(&mut world);

    let element_manager: ElementManager = ElementManager::new();
    register_element_groups(&element_manager);

    loop {
        game_properties.left_mouse_down = if is_mouse_button_down(MouseButton::Left) {
            true
        } else {
            false
        };
        game_properties.right_mouse_down = if is_mouse_button_down(MouseButton::Right) {
            true
        } else {
            false
        };

        world_info.fps = get_fps();

        let w = image.width();
        let h = image.height();

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0 as usize;
        let mouse_y = mouse_pos.1 as usize;

        // convert screen coords to world coords for mouse
        let (screen_w, screen_h) = screen_size();
        let mouse_x_world = (mouse_x as f32 / (screen_w - 200.) * w as f32) as usize;
        let mouse_y_world = (mouse_y as f32 / (screen_h - 60.) * h as f32) as usize;

        /*
        for x in 0..w as u32 {
            for y in 0..h as u32 {
                let particle = world.get_particle(x as i32, y as i32);
                let color = particle_to_color(particle.variant);
                let c = color_u8!(color.0, color.1, color.2, 255);

                image.set_pixel(x, y, c);
            }
        }
        */

        // use parallel iterator to speed up rendering

        image
            .get_image_data_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let x = idx % w as usize;
                let y = idx / w as usize;
                let particle = world.get_particle(x as i32, y as i32);
                let color = particle_to_color(particle.variant);
                let c = color_u8!(color.0, color.1, color.2, 255);
                *pixel = [color.0, color.1, color.2, 255];
            });

        //** */
        if world.cleared {
            world.reset();
        } else if !world.modified_indices.is_empty() {
            // go through modified indices and only update those on the image
            for idx in world.modified_indices.iter() {
                let x = idx % w as usize;
                let y = idx / w as usize;
                let particle = world.get_particle(x as i32, y as i32);
                let color = particle_to_color(particle.variant);
                let c = color_u8!(color.0, color.1, color.2, 255);
                image.set_pixel(x as u32, y as u32, c);
            }
        }

        if mouse_position().0 < screen_w - 200. && mouse_position().1 < screen_h - 60. {
            let particle = world.get_particle(mouse_x_world as i32, mouse_y_world as i32);
            world_info.properties.hovering_over = particle;
            world_info.properties.hovering_temperature =
                world.get_temperature(mouse_x_world as i32, mouse_y_world as i32);
        }
        if game_properties.left_mouse_down || game_properties.right_mouse_down {}

        world.tick();

        let mouse_wheel = mouse_wheel().1;
        //world_info.properties.tool_radius += mouse_wheel;
        // use logarithmic scale for radius change
        if mouse_wheel > 0.0 {
            world_info.properties.tool_radius *= 1.1;
        } else if mouse_wheel < 0.0 {
            world_info.properties.tool_radius /= 1.1;
        }
        // handle input
        if game_properties.left_mouse_down
            && mouse_y < screen_h as usize - 60
            && mouse_x < screen_w as usize - 200
        {
            // use screen coords mapped to world coords
            // make sure that the particle at the mouse position is empty

            if let Tool::ElementTool(variant) = world_info.properties.tool_type {
                paint_radius(
                    &mut world,
                    mouse_x_world as i32,
                    mouse_y_world as i32,
                    variant,
                    world_info.properties.tool_radius as i32,
                );
            } else {
                // If the tool is not an ElementTool, use the tool directly
                use_tool(
                    world_info.properties,
                    &mut world,
                    mouse_x_world as i32,
                    mouse_y_world as i32,
                );
            }
        }

        if game_properties.right_mouse_down {
            erase_radius(
                &mut world,
                mouse_x_world as i32,
                mouse_y_world as i32,
                world_info.properties.tool_radius as i32,
            );
        }

        // check if mouse is moving
        #[cfg(target_arch = "wasm32")]
        for touch in touches() {
            world.resume();

            if touch.position.y < screen_h - 60. && touch.position.x < screen_w - 200. {
                // use screen coords mapped to world coords
                // make sure that the particle at the mouse position is empty
                if world
                    .get_particle(touch.position.x as i32, touch.position.y as i32)
                    .variant
                    == Variant::Empty
                {
                    paint_radius(
                        &mut world,
                        touch.position.x as i32,
                        touch.position.y as i32,
                        world_info.properties.tool_type,
                        world_info.properties.tool_radius as i32,
                    );
                }
            }
        }

        // if z is pressed, draw a zoomed in version of the world at the mouse position inside a rect

        if is_key_pressed(KeyCode::R) {
            world.cleared = true;
        }

        if is_key_pressed(KeyCode::Space) {
            if world.running {
                world.pause();
            } else {
                world.resume();
            }
        }

        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width() - 200., screen_height() - 60.)),
                source: Some(Rect::new(0.0, 0.0, w as f32, h as f32)),

                ..Default::default()
            },
        );

        draw_top_panel(&mut world_info);
        draw_tool_outline(&mut world_info);
        draw_group_sidebar(&element_manager, &mut world_info);
        draw_element_list(&element_manager, &mut world_info);

        next_frame().await
    }
}

fn register_element_groups(manager: &ElementManager) {
    manager.register_group("Powders", vec![Variant::Sand, Variant::Salt]);
    manager.register_group("Liquids", vec![Variant::Water, Variant::SaltWater]);
    manager.register_group("Gases", vec![Variant::Smoke, Variant::CO2]);
    manager.register_group("Explosives", vec![Variant::Fire]);
    manager.register_group("Walls", vec![Variant::Wall]);
    manager.register_group(
        "PHYS",
        vec![
            Variant::CARB,
            Variant::IRON,
            Variant::OXGN,
            Variant::HYGN,
            Variant::HELM,
            Variant::NITR,
        ],
    )
}
