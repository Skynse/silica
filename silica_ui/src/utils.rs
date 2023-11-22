use ::rand::Rng;
use macroquad::miniquad::window::cancel_quit;
use macroquad::prelude::*;
use macroquad::ui::widgets;
use macroquad::ui::{hash, root_ui};

use silica_engine::{group::ElementManager, variant::Variant, world::World};

use crate::data::get_save_dir;
use crate::manager::{GameProperties, Property, Tool, WorldInfo};
use crate::{TOOLS, UI_OFFSET_X, UI_OFFSET_Y};

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
    root_ui().window(
        hash!(),
        vec2(screen_width() - UI_OFFSET_X, 30.0),
        vec2(UI_OFFSET_X, screen_height() - 30.0),
        |ui| {
            for (idx, group) in manager.groups.borrow().iter().enumerate() {
                let button = widgets::Button::new(group.group_name.clone())
                    .position(vec2(0.0, 30.0 * (idx + 1) as f32))
                    .size(vec2(UI_OFFSET_X, 30.0))
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
    let button_size = 50.;

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
            let y = 0.0;
            for e in elements {
                let button = widgets::Button::new(e.get_name())
                    .position(vec2(x, y))
                    .size(vec2(button_size, button_size))
                    .selected(world_info.properties.tool_type == Tool::ElementTool(e))
                    .ui(ui);

                if world_info.properties.tool_type == Tool::ElementTool(e) {
                    draw_rectangle_lines(
                        x,
                        screen_height() - UI_OFFSET_Y + 30.0,
                        button_size,
                        button_size,
                        5.0,
                        Color::new(1.0, 0., 0., 1.0),
                    );
                }

                // COLOR button
                if button {
                    world_info.properties.tool_type = Tool::ElementTool(e);
                }
                x += button_size;
            }
        },
    );
}

// for tools
pub fn draw_bottom_panel(world_info: &mut WorldInfo, _gameprops: &mut GameProperties) {
    // draw right above the element panel

    let panel_height = 30.0; // Adjust the height of the top panel as needed
    let button_width: f32 = 50.0;
    // draw some buttons for the three tools, selectable too

    root_ui().window(
        hash!(),
        vec2(0.0, screen_height() - UI_OFFSET_Y - 30.),
        vec2(screen_width(), panel_height + 30.),
        |ui| {
            let mut x = 0.0;
            let y = 0.0;
            for p in TOOLS {
                let button = widgets::Button::new(p.to_string())
                    .position(vec2(x, y))
                    .selected(world_info.properties.tool_type == Tool::PropertyTool(p))
                    .size(vec2(button_width, panel_height))
                    .ui(ui);
                if button {
                    match p {
                        Property::Temperature => {
                            world_info.properties.tool_type =
                                Tool::PropertyTool(Property::Temperature);
                        }
                        Property::COOL => {
                            world_info.properties.tool_type = Tool::PropertyTool(Property::COOL);
                        }
                        Property::Pressure => {
                            world_info.properties.tool_type =
                                Tool::PropertyTool(Property::Pressure);
                        }

                        Property::DelWall => {
                            world_info.properties.tool_type = Tool::PropertyTool(Property::DelWall);
                        }
                    }
                }
                x += button_width;
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
        &format!("FPS: {:.2}\nParts: {}", world_info.fps, world_info.parts), // Display FPS with two decimal places
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
        &format!(
            "{:?}",
            world_info
                .properties
                .hovering_over
                .variant_type
                .source_variant
        ),
        screen_width() - 200.0 - 200.,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );

    // write temperature
    let _temp = world_info.properties.hovering_over.temperature;

    draw_text(
        &format!("AH: {:.2}C", world_info.properties.hovering_temperature),
        screen_width() - 150.0,
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

                let _probability_to_draw = ::rand::thread_rng().gen_range(0..100);

                world.set_particle(particle_x as i32, particle_y as i32, variant);
            }
        }
    }
}

pub fn use_tool(props: GameProperties, world: &mut World, x: i32, y: i32) {
    match props.tool_type {
        Tool::ElementTool(variant) => {
            let radius: i32 = props.tool_radius as i32;
            paint_radius(world, x, y, variant, radius);
        }
        Tool::PropertyTool(property) => match property {
            Property::Temperature => {
                //world.set_temperature(x, y, 100.0);
                let radius: i32 = props.tool_radius as i32;
                for dx in -radius..radius {
                    for dy in -radius..radius {
                        let distance_squared = (dx * dx + dy * dy) as f32;
                        let _radius_squared = radius as f32 * radius as f32;

                        if distance_squared > _radius_squared {
                            continue;
                        }
                        let sigm: f32 = 10.;
                        let exp = -distance_squared / (2.0 * sigm.powi(2));
                        // intensity based on radius

                        world.add_heat(x + dx, y + dy, 40. * exp.exp());
                    }
                }
            }
            Property::Pressure => {
                //world.set_pressure(x, y, 100);
                let radius: i32 = props.tool_radius as i32;
                for dx in -radius..radius {
                    for dy in -radius..radius {
                        let distance_squared = (dx * dx + dy * dy) as f32;
                        let _radius_squared = radius as f32 * radius as f32;
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
                        let _radius_squared = radius as f32 * radius as f32;

                        if distance_squared > _radius_squared {
                            continue;
                        }
                        let sigm: f32 = 10.;
                        let exp = -distance_squared / (2.0 * sigm.powi(2));

                        world.add_heat(x + dx, y + dy, -40. * exp.exp());
                    }
                }
            }

            Property::DelWall => {
                let radius: i32 = props.tool_radius as i32;
                erase_indestructible(world, x, y, radius);
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

pub fn erase_indestructible(world: &mut World, x: i32, y: i32, radius: i32) {
    for dx in -radius..radius {
        for dy in -radius..radius {
            if dx * dx + dy * dy < radius * radius {
                world.erase_indestructible(dx + x, dy + y);
            }
        }
    }
}

pub fn draw_confirm_exit(mut props: GameProperties) {
    root_ui().window(
        hash!(),
        vec2(screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 50.0),
        vec2(200.0, 100.0),
        |ui| {
            widgets::Label::new("Are you sure you want to exit?")
                .position(vec2(0.0, 0.0))
                .size(vec2(200.0, 50.0))
                .ui(ui);

            if widgets::Button::new("Yes")
                .position(vec2(0.0, 50.0))
                .size(vec2(100.0, 50.0))
                .ui(ui)
            {
                std::process::exit(0);
            }

            if widgets::Button::new("No")
                .position(vec2(100.0, 50.0))
                .size(vec2(100.0, 50.0))
                .ui(ui)
            {
                props.requested_exit = false;
            }
        },
    );
}

struct OpenDialog {
    open: bool,
    path: String,
}

// open dialog shows a list of files in the save directory
// on top is a text box to enter a new filename to filter the list
// the contents of this box will be the images representing the png saves
// for this we will have to create textures from the pngs using macroquad
