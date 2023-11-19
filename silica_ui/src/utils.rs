use ::rand::Rng;
use macroquad::prelude::*;
use macroquad::ui::widgets;
use macroquad::ui::{hash, root_ui, Skin};
use silica_engine::variant;
use silica_engine::{group::ElementManager, variant::Variant, world::World};

use crate::manager::{GameProperties, Property, Tool, WorldInfo};

pub fn draw_walls(world: &mut World) {
    for x in 0..world.width {
        world.set_particle(x as i32, 0, Variant::Wall);
        world.set_particle(x as i32, world.height as i32 - 1, Variant::Wall);
    }
    for y in 0..world.height {
        world.set_particle(0, y as i32, Variant::Wall);
        world.set_particle(world.width as i32 - 1, y as i32, Variant::Wall);
    }
}

pub fn draw_group_sidebar(manager: &ElementManager, world_info: &mut WorldInfo) {
    let mut ui = root_ui().window(
        hash!(),
        vec2(screen_width() - 200.0, 30.0),
        vec2(200.0, screen_height() - 30.0),
        |ui| {
            ui.label(None, "Groups");
            for (idx, group) in manager.groups.borrow().iter().enumerate() {
                let button = widgets::Button::new(group.group_name.clone())
                    .position(vec2(0.0, 30.0 * (idx + 1) as f32))
                    .size(vec2(200.0, 30.0))
                    .ui(ui);
                if button {
                    world_info.properties.selected_group_idx = idx;
                }
            }
        },
    );
}

// horizontal left to right assortment of elements
// shows up on bottom of screen
// truncated names to 4 chars
pub fn draw_tool_outline(world_info: &mut WorldInfo) {
    // Get mouse position
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_x = mouse_x as f32;
    let mouse_y = mouse_y as f32;

    let radius = world_info.properties.tool_radius;

    draw_circle_lines(
        mouse_x,
        mouse_y,
        radius,
        2.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
}

pub fn draw_element_list(manager: &ElementManager, world_info: &mut WorldInfo) {
    let button_size = 60.0;

    root_ui().window(
        hash!(),
        vec2(0.0, screen_height() - button_size),
        vec2(screen_width(), 30.0 + button_size),
        |ui| {
            let binding = manager.groups.borrow();
            let group = binding
                .get(world_info.properties.selected_group_idx)
                .unwrap();
            let elements = group.get_elements();
            let mut x = 0.0;
            for element in elements {
                let button = widgets::Button::new(element.get_name())
                    .position(vec2(x, 0.0))
                    .size(vec2(100.0, 60.0))
                    .ui(ui);

                // use the variant color to draw a rectangle over the button
                let color = variant::variant_type(element).color.to_rgba8();
                draw_rectangle(
                    x,
                    screen_height() - 30.0,
                    100.0,
                    30.0,
                    Color::new(
                        color.0 as f32 / 255.0,
                        color.1 as f32 / 255.0,
                        color.2 as f32 / 255.0,
                        0.5,
                    ),
                );
                // if button is selected, draw a rectangle around it
                if world_info.properties.tool_type.get_variant() == Some(element) {
                    draw_rectangle_lines(
                        x,
                        screen_height() - 30.0,
                        100.0,
                        30.0,
                        2.0,
                        Color::new(1.0, 1.0, 1.0, 1.0),
                    );
                }
                if button {
                    world_info.properties.tool_type = Tool::ElementTool(element);
                }
                x += 100.0;
            }
        },
    );

    // while we're at it, lets also add the property buttons
    root_ui().window(
        hash!(),
        vec2(0.0, screen_height() - button_size - 30.0),
        vec2(screen_width(), 30.0),
        |ui| {
            let button = widgets::Button::new("HEAT")
                .position(vec2(0.0, 0.0))
                .size(vec2(100.0, 30.0))
                .ui(ui);
            if button {
                world_info.properties.tool_type = Tool::PropertyTool(Property::Temperature);
            }
            let button = widgets::Button::new("Pressure")
                .position(vec2(100.0, 0.0))
                .size(vec2(100.0, 30.0))
                .ui(ui);
            if button {
                world_info.properties.tool_type = Tool::PropertyTool(Property::Pressure);
            }
        },
    );
}
pub fn draw_top_panel(world_info: &mut WorldInfo) {
    let panel_height = 30.0; // Adjust the height of the top panel as needed

    // Draw a colored rectangle at the top to represent the panel
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        panel_height,
        Color::new(50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 0.0), // Adjust the color as needed
    );

    // Draw text or other UI elements on the top panel
    draw_text(
        &format!("FPS: {:.2}", world_info.fps), // Display FPS with two decimal places
        20.0,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );

    // draw currently selected tool to the right
    let variant = world_info.properties.tool_type.get_variant();
    match variant {
        Some(variant) => variant.to_string(),
        None => "None".to_string(),
    };
    draw_text(
        &format!("{:?}", world_info.properties.hovering_over.variant),
        screen_width() - 200.0 - 200.,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );

    // write temperature
    let temp = world_info.properties.hovering_over.temperature;

    draw_text(
        &format!("{:.2}C", world_info.properties.hovering_temperature),
        screen_width() - 100.0,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );
    let temp = world_info.properties.hovering_over.temperature;

    // write temperature of the cell itself
    draw_text(
        &format!("{:.2}C", temp),
        screen_width() - 200.0 - 100.,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );
}

// draw thermometer

pub fn paint_radius(world: &mut World, x: i32, y: i32, variant: Variant, radius: i32) {
    let center_x = x as f32 + 0.1; // Adding 0.5 for a half-pixel offset to center particles
    let center_y = y as f32 + 0.1;

    for dx in -radius..radius {
        for dy in -radius..radius {
            let distance_squared = (dx * dx + dy * dy) as f32;
            let radius_squared = radius as f32 * radius as f32;

            if distance_squared < radius_squared {
                let particle_x = center_x + dx as f32;
                let particle_y = center_y + dy as f32;
                let probability_to_draw = ::rand::thread_rng().gen_range(0..100);
                if probability_to_draw < 50 && variant != Variant::Wall {
                    continue;
                }
                world.set_particle(particle_x as i32, particle_y as i32, variant);
            }
        }
    }
}

pub fn use_tool(props: GameProperties, world: &mut World, x: i32, y: i32) {
    match props.tool_type {
        Tool::ElementTool(variant) => {
            world.set_particle(x, y, variant);
        }
        Tool::PropertyTool(property) => match property {
            Property::Temperature => {
                //world.set_temperature(x, y, 100.0);
                let radius: i32 = props.tool_radius as i32;
                for dx in -radius..radius {
                    for dy in -radius..radius {
                        let distance_squared = (dx * dx + dy * dy) as f32;
                        let radius_squared = radius as f32 * radius as f32;
                        let sigm: f32 = 0.5;
                        let exp = -distance_squared / (2.0 * sigm.powi(2));

                        world.add_heat(x + dx, y + dy, 100. * exp.exp());
                    }
                }
            }
            Property::Pressure => {
                //world.set_pressure(x, y, 100);
                let radius: i32 = props.tool_radius as i32;
                for dx in -radius..radius {
                    for dy in -radius..radius {
                        let distance_squared = (dx * dx + dy * dy) as f32;
                        let radius_squared = radius as f32 * radius as f32;
                        let sigm: f32 = 0.5;
                        let exp = -distance_squared / (2.0 * sigm.powi(2));

                        world.set_pressure(x + dx, y + dy, 100. * exp.exp());
                    }
                }
            }

            Property::COOL => {
                //world.set_pressure(x, y, 100);
                let radius: i32 = props.tool_radius as i32;
                for dx in -radius..radius {
                    for dy in -radius..radius {
                        let distance_squared = (dx * dx + dy * dy) as f32;
                        let radius_squared = radius as f32 * radius as f32;
                        let sigm: f32 = 0.5;
                        let exp = -distance_squared / (2.0 * sigm.powi(2));

                        world.add_heat(x + dx, y + dy, -100. * exp.exp());
                    }
                }
            }
        },
    }
}

pub fn erase_radius(world: &mut World, x: i32, y: i32, radius: i32) {
    for dx in -radius..radius {
        for dy in -radius..radius {
            if dx * dx + dy * dy < radius * radius {
                world.set_particle(x + dx, y + dy, Variant::Empty);
            }
        }
    }
}
