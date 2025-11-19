mod data;
mod improved_wind;
mod manager;
mod utils;
mod wind_viz;

use data::{create_data_dir, get_save_dir};
use macroquad::miniquad::window::{request_quit, screen_size};
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin, Style};
use manager::{GameProperties, Property, RenderMode, Tool, WorldInfo};
use rayon::prelude::*;
use silica_engine::group::ElementManager;
use silica_engine::prelude::*;
use utils::*;

const UI_OFFSET_X: f32 = 50.0;
const UI_OFFSET_Y: f32 = 60.;

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

const TOOLS: [Property; 5] = [
    Property::Temperature,
    Property::COOL,
    Property::Pressure,
    Property::DelWall,
    Property::Wind,
];

#[macroquad::main(window_conf)]
async fn main() {
    prevent_quit();
    create_data_dir();
    let label_style: Style = root_ui()
        .style_builder()
        .font(include_bytes!("./fonts/standard.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(0, 180, 120, 255))
        .font_size(30)
        .build();

    let window_style: Style = root_ui()
        .style_builder()
        .color(Color::from_rgba(0, 0, 0, 0))
        .build();
    let skin = Skin {
        label_style,
        window_style,
        ..root_ui().default_skin()
    };
    let _pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    // Initialization

    let w: usize = 611;
    let h: usize = 383;
    let mut image = Image::gen_image_color(w as u16, h as u16, color_u8!(13, 16, 20, 1));
    let mut world: World = World::new(w as i32, h as i32);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut chosen_name = String::new();
    let mut filter_name = String::new();

    let mut game_properties = GameProperties {
        tool_radius: 10.0,
        tool_type: Tool::ElementTool(Variant::Sand),
        selected_group_idx: 0,
        hovering_over: Particle::default(),
        hovering_temperature: 0.0,
        left_mouse_down: false,
        right_mouse_down: false,
        render_mode: RenderMode::Normal,
        requested_exit: false,
        requested_save: false,
        requested_load: false,
    };
    let mut world_info = WorldInfo {
        fps: 0.,
        properties: game_properties,
        world_width: w,
        world_height: h,
        parts: 0,
    };

    draw_walls(&mut world);

    let element_manager: ElementManager = ElementManager::new();
    register_element_groups(&element_manager);
    root_ui().push_skin(&skin);

    loop {
        clear_background(BLACK);
        let can_draw = !game_properties.requested_exit
            && !game_properties.requested_save
            && !game_properties.requested_load;

        if is_key_pressed(KeyCode::Escape) {
            // we could be loading, saving, or trying to exit
            if game_properties.requested_load || game_properties.requested_save {
                game_properties.requested_load = false;
                game_properties.requested_save = false;
            } else {
                game_properties.requested_exit = !game_properties.requested_exit;
            }
        }

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

        let w = image.width();
        let h = image.height();

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0 as usize;
        let mouse_y = mouse_pos.1 as usize;

        // convert screen coords to world coords for mouse
        let (screen_w, screen_h) = screen_size();
        let mouse_x_world = (mouse_x as f32 / (screen_w - UI_OFFSET_X) * w as f32) as usize;
        let mouse_y_world = (mouse_y as f32 / (screen_h - UI_OFFSET_Y) * h as f32) as usize;

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
        if game_properties.render_mode == RenderMode::Heat {
            // draw temperature map of World environment
            for x in 0..w as u32 {
                for y in 0..h as u32 {
                    let temp = world.get_temperature(x as i32, y as i32);
                    let part_temp = world.get_particle(x as i32, y as i32).temperature;
                    let color = temp_to_color(temp + part_temp);
                    let c = color_u8!(color.0, color.1, color.2, 255);

                    image.set_pixel(x, y, c);
                }
            }
        } else if game_properties.render_mode == RenderMode::Wind {
            // draw wind visualization with improved colors and performance
            for x in 0..w as u32 {
                for y in 0..h as u32 {
                    let wind = world.get_wind(x as i32, y as i32);
                    let particle = world.get_particle(x as i32, y as i32);

                    // Calculate wind strength more efficiently
                    let wind_strength = wind.strength(); // Enhance wind visualization - show more wind effects
                                                         // Lower the threshold for visualization to show more subtle wind
                    if particle.variant_type.source_variant == Variant::Empty || wind_strength > 1.0
                    {
                        // Map wind strength to color intensity (blue for right/down, red for left/up)
                        // Further increase the multiplier to make wind colors more vibrant
                        let intensity = (wind_strength * 40.0).min(255.0) as u8;

                        // Add a minimum intensity for better visibility of subtle wind
                        let min_visible = if game_properties.render_mode == RenderMode::Wind {
                            18 // Higher minimum in wind mode
                        } else {
                            12 // Standard minimum in other modes
                        };

                        // Improved color scheme for wind visualization
                        let (r, g, b) = if wind.dx.abs() > wind.dy.abs() {
                            // Horizontal wind is stronger
                            if wind.dx > 0 {
                                // Right wind -> blue
                                (0, 0, intensity)
                            } else {
                                // Left wind -> red
                                (intensity, 0, 0)
                            }
                        } else if wind.dy != 0 {
                            // Vertical wind is stronger
                            if wind.dy > 0 {
                                // Down wind -> cyan
                                (0, intensity, intensity)
                            } else {
                                // Up wind -> yellow
                                (intensity, intensity, 0)
                            }
                        } else {
                            // No wind
                            (0, 0, 0)
                        }; // If there's a particle, blend the wind color with the particle color
                        if particle.variant_type.source_variant != Variant::Empty {
                            let particle_color = particle_to_color(particle);

                            // Enhanced adaptive wind visibility based on strength and mode
                            // Make wind more visible in Wind mode specifically
                            let base_factor = if game_properties.render_mode == RenderMode::Wind {
                                (wind_strength / 6.0).min(0.8) // Higher visibility in wind mode
                            } else {
                                (wind_strength / 10.0).min(0.7) // Normal visibility
                            };

                            // Blend particle color with wind color
                            let blended_r = ((1.0 - base_factor) * particle_color.r as f32
                                + base_factor * r as f32)
                                as u8;
                            let blended_g = ((1.0 - base_factor) * particle_color.g as f32
                                + base_factor * g as f32)
                                as u8;
                            let blended_b = ((1.0 - base_factor) * particle_color.b as f32
                                + base_factor * b as f32)
                                as u8;

                            image.set_pixel(x, y, color_u8!(blended_r, blended_g, blended_b, 255));
                        } else {
                            // Enhanced wind visualization for empty cells
                            // Make empty cells more distinctive in Wind mode
                            if game_properties.render_mode == RenderMode::Wind {
                                // Enhance the color intensity for better visibility in Wind mode
                                let enhanced_r = (r as f32 * 1.2).min(255.0) as u8;
                                let enhanced_g = (g as f32 * 1.2).min(255.0) as u8;
                                let enhanced_b = (b as f32 * 1.2).min(255.0) as u8; // Add stronger background tint to make wind more visible in empty space
                                let background = 25; // Even higher value for better visibility in dark areas

                                // Apply a more pronounced color effect
                                image.set_pixel(
                                    x,
                                    y,
                                    color_u8!(
                                        enhanced_r.max(background),
                                        enhanced_g.max(background),
                                        enhanced_b.max(background),
                                        255
                                    ),
                                );
                            } else {
                                // Just use the standard wind color for empty cells in other modes
                                image.set_pixel(x, y, color_u8!(r, g, b, 255));
                            }
                        }
                    } else {
                        // Default to normal particle rendering for non-empty cells with minimal wind
                        let color = particle_to_color(particle);
                        image.set_pixel(x, y, color_u8!(color.r, color.g, color.b, 255));
                    }
                }
            } // Draw wind direction arrows for stronger wind (on a grid)            // Calculate optimal grid size based on resolution and render mode
              // Use a denser grid in Wind visualization mode for more detailed visualization
            let base_divisor = if game_properties.render_mode == RenderMode::Wind {
                // Denser grid in Wind mode
                if w > 800 || h > 800 {
                    36.0
                } else {
                    30.0
                }
            } else {
                // Standard grid in other modes
                if w > 800 || h > 800 {
                    42.0
                } else {
                    36.0
                }
            };
            let grid_size = (w.min(h) as f32 / base_divisor).round().max(4.0) as u32;

            // Use staggered grid for more natural-looking patterns
            let mut y_offset = 0;

            for grid_x in (grid_size / 2..w as u32).step_by(grid_size as usize) {
                // Stagger rows for more natural distribution
                y_offset = (y_offset + grid_size / 2) % grid_size;

                for grid_y in ((grid_size / 2 + y_offset)..h as u32).step_by(grid_size as usize) {
                    let wind = world.get_wind(grid_x as i32, grid_y as i32);
                    let wind_strength = wind.strength();

                    // Adaptive threshold based on render mode
                    // Lower threshold to show more arrows with higher density in Wind mode
                    let threshold = if game_properties.render_mode == RenderMode::Wind {
                        0.2 // Even lower threshold to show more wind arrows in Wind mode
                    } else {
                        0.5 // Standard threshold in other modes
                    };

                    // Only draw arrows for wind above the threshold
                    if wind_strength > threshold {
                        // Draw wind indicator with improved visibility
                        draw_wind_arrow(
                            &mut image,
                            grid_x,
                            grid_y,
                            wind.dx,
                            wind.dy,
                            wind_strength,
                        );
                    }
                }
            }
        } else {
            // Normal mode
            image
                .get_image_data_mut()
                .par_iter_mut()
                .enumerate()
                .for_each(|(idx, pixel)| {
                    let x = idx % w as usize;
                    let y = idx / w as usize;
                    let particle = world.get_particle(x as i32, y as i32);

                    let color = particle_to_color(particle);
                    *pixel = [color.r, color.g, color.b, 255]
                });

            /* */
            if world.cleared && can_draw {
                world.reset();
            } else if !world.modified_indices.is_empty() {
                // go through modified indices and only update those on the image
                for idx in world.modified_indices.iter() {
                    let x = idx % w as usize;
                    let y = idx / w as usize;
                    let particle = world.get_particle(x as i32, y as i32);

                    let color = particle_to_color(particle).to_rgba8();

                    let c = color_u8!(color.0, color.1, color.2, color.3);
                    image.set_pixel(x as u32, y as u32, c);
                }
            }
        }

        world_info.parts = world.get_particle_count();
        if mouse_position().0 < screen_w - UI_OFFSET_X
            && mouse_position().1 < screen_h - UI_OFFSET_Y
        {
            let particle = world.get_particle(mouse_x_world as i32, mouse_y_world as i32);
            world_info.properties.hovering_over = particle;
            world_info.properties.hovering_temperature =
                world.get_temperature(mouse_x_world as i32, mouse_y_world as i32);
        }
        if game_properties.left_mouse_down || game_properties.right_mouse_down {}

        if game_properties.requested_exit
            || game_properties.requested_save
            || game_properties.requested_load
        {
            // don't update the world
            world.pause();
        } else {
            world.resume();
        }
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
        if can_draw
            && game_properties.left_mouse_down
            && mouse_y < screen_h as usize - UI_OFFSET_Y as usize
            && mouse_x < screen_w as usize - UI_OFFSET_X as usize
        {
            // use screen coords mapped to world coords
            // make sure that the particle at the mouse position is empty

            if let Tool::ElementTool(_) = world_info.properties.tool_type {
                use_tool(
                    world_info.properties,
                    &mut world,
                    mouse_x_world as i32,
                    mouse_y_world as i32,
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
                    use_tool(
                        world_info.properties,
                        &mut world,
                        touch.position.x as i32,
                        touch.position.y as i32,
                    );
                }
            }
        }

        // if z is pressed, draw a zoomed in version of the world at the mouse position inside a rect

        if is_key_pressed(KeyCode::R) {
            world.cleared = true;
        }

        if is_key_pressed(KeyCode::Space) {
            world.running = !world.running;
        }

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Key1 => {
                    game_properties.render_mode = RenderMode::Normal;
                }
                KeyCode::Key2 => {
                    game_properties.render_mode = RenderMode::Heat;
                }

                KeyCode::Key3 => {
                    game_properties.render_mode = RenderMode::Wind;
                }

                _ => (),
            }
        }

        // CTRL + S to save
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S) {
            game_properties.requested_save = true;
        }

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::L) {
            game_properties.requested_load = true;
        }

        if game_properties.requested_load {
            // show a dialog with a filter for the save files
            // each save file should show a mini preview of the png file
            // when the user clicks on a save file, load it

            let dialog_width = screen_width() * 0.6; // Adjust the width of the dialog
            let dialog_height = screen_height() * 0.6; // Adjust the height of the dialog

            let button_height = 50.0; // Set the height of the buttons
            let cancel_button_position =
                vec2((dialog_width / 2.0), dialog_height - button_height - 20.0); // Adjust the position of the Cancel button

            root_ui().window(
                hash!(),
                vec2(
                    (screen_width() - dialog_width) / 2.0,
                    (screen_height() - dialog_height) / 2.0,
                ), // Center the dialog on the screen
                vec2(dialog_width, dialog_height),
                |ui| {
                    // Input Box
                    ui.input_text(hash!(), "Filename", &mut filter_name);

                    // filter through the files and list them so that we can click on one
                    let mut save_dir = get_save_dir();
                    save_dir.push(&filter_name);
                    let mut x: f32 = 0.;
                    let mut y: f32 = 0.;

                    // iterate through the save dir, by index, so we can get the image by index
                    if let Ok(dir) = std::fs::read_dir(save_dir) {
                        for entry in dir {
                            if let Ok(entry) = entry {
                                let path = entry.path();
                                let file_name = path.file_name().unwrap().to_str().unwrap();

                                // check if the file is a png
                                if file_name.ends_with(".png") {
                                    // draw the image
                                    let img = load_img(path.to_str().unwrap());
                                    // we need to resize the image to 1/4 of the original size
                                    let img_width = img.width() / 8;
                                    let img_height = img.height() / 8;
                                    let img_texture = Texture2D::from_image(&img);
                                    img_texture.set_filter(FilterMode::Linear);
                                    draw_texture_ex(
                                        &img_texture,
                                        screen_width() / 2.0 - dialog_width / 2.0 + x as f32,
                                        30. + screen_height() / 2.0 - dialog_height / 2.0
                                            + y as f32,
                                        WHITE,
                                        DrawTextureParams {
                                            dest_size: Some(vec2(
                                                img_width as f32,
                                                img_height as f32,
                                            )),
                                            ..Default::default()
                                        },
                                    );

                                    // check if the image is clicked
                                    if is_mouse_button_pressed(MouseButton::Left)
                                        && mouse_position().0
                                            > screen_width() / 2.0 - dialog_width / 2.0 + x as f32
                                        && mouse_position().0
                                            < screen_width() / 2.0 - dialog_width / 2.0
                                                + x as f32
                                                + img_width as f32
                                        && mouse_position().1
                                            > 30. + screen_height() / 2.0 - dialog_height / 2.0
                                                + y as f32
                                        && mouse_position().1
                                            < 30. + screen_height() / 2.0 - dialog_height / 2.0
                                                + y as f32
                                                + img_height as f32
                                    {
                                        // load the image
                                        let mut save_dir = get_save_dir();
                                        save_dir.push(file_name.replace(".png", ".slc"));
                                        world.load_from_slc(save_dir.to_str().unwrap());
                                        game_properties.requested_load = false;
                                        filter_name.clear();
                                    }

                                    // increment x and y
                                    if x + img_width as f32 > dialog_width {
                                        y += img_height as f32;
                                        x = 0.;
                                    } else {
                                        x += img_width as f32;
                                    }
                                }
                            }
                        }
                    }

                    // Cancel Button
                    if widgets::Button::new("Cancel")
                        .position(cancel_button_position)
                        .size(vec2(dialog_width / 2.0 - 20.0, button_height))
                        .ui(ui)
                    {
                        game_properties.requested_load = false;
                    }
                },
            );
        }

        if game_properties.requested_save {
            let dialog_width = screen_width() * 0.6; // Adjust the width of the dialog
            let dialog_height = screen_height() * 0.6; // Adjust the height of the dialog

            let input_box_height = 50.0; // Set the height of the input box
            let button_height = 50.0; // Set the height of the buttons

            let input_box_position = vec2(dialog_width / 2.0, 20.0); // Adjust the position of the input box
            let save_button_position =
                vec2(dialog_width / 4.0, dialog_height - button_height - 20.0); // Adjust the position of the Save button
            let cancel_button_position = vec2(
                (dialog_width / 4.0) * 3.0,
                dialog_height - button_height - 20.0,
            ); // Adjust the position of the Cancel button

            root_ui().window(
                hash!(),
                vec2(
                    (screen_width() - dialog_width) / 2.0,
                    (screen_height() - dialog_height) / 2.0,
                ), // Center the dialog on the screen
                vec2(dialog_width, dialog_height),
                |ui| {
                    // Input Box
                    ui.input_text(hash!(), "Filename", &mut chosen_name);

                    // Save Button
                    if widgets::Button::new("Save")
                        .position(save_button_position)
                        .size(vec2(dialog_width / 2.0 - 20.0, button_height))
                        .ui(ui)
                    {
                        // save the file
                        let mut save_dir = get_save_dir();
                        save_dir.push(&chosen_name);
                        world.save(save_dir.to_str().unwrap());
                        let path = save_dir.to_str().unwrap();
                        let fname = format!("{}.png", path);
                        // concat chosen_name with .png and the save dir

                        image.export_png(&fname);
                        world.save_to_slc(path);
                        game_properties.requested_save = false;
                        chosen_name.clear();
                    }

                    // Cancel Button
                    if widgets::Button::new("Cancel")
                        .position(cancel_button_position)
                        .size(vec2(dialog_width / 2.0 - 20.0, button_height))
                        .ui(ui)
                    {
                        game_properties.requested_save = false;
                    }
                },
            );
        }

        if game_properties.requested_exit {
            draw_confirm_exit(game_properties);
        }

        texture.update(&image);
        if can_draw {
            draw_texture_ex(
                &texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        screen_width() - UI_OFFSET_X,
                        screen_height() - UI_OFFSET_Y, //ADJUST BASED ON BUTTON SIZE,
                    )),
                    source: Some(Rect::new(0.0, 0.0, w as f32, h as f32)),

                    ..Default::default()
                },
            );
        }
        gl_use_default_material();

        draw_top_panel(&mut world_info);
        draw_bottom_panel(&mut world_info, &mut game_properties);
        if can_draw {
            draw_tool_outline(&mut world_info);
        }

        draw_group_sidebar(&element_manager, &mut world_info);
        draw_element_list(&element_manager, &mut world_info);
        let end = get_time();

        // every half a second, update the fps counter
        if world.generation % 10 == 0 {
            world_info.fps = get_fps() as f32;
        }
        next_frame().await
    }
}

fn register_element_groups(manager: &ElementManager) {
    manager.register_group("PWDR", vec![Variant::Sand, Variant::Salt]);
    manager.register_group("FLUID", vec![Variant::Water, Variant::SaltWater]);
    manager.register_group("GAS", vec![Variant::Smoke, Variant::CO2, Variant::WTVP]);
    manager.register_group("EXPV", vec![Variant::Fire]);
    manager.register_group("SOLID", vec![Variant::Glass]);
    manager.register_group("WALL", vec![Variant::Wall]);
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
    );
    manager.register_group("Life", vec![Variant::GOL]);
    // Wind is now a property tool, not a physical element
    manager.register_group("MECH", vec![])
}

pub fn temp_to_color(temperature: f32) -> (u8, u8, u8) {
    // red is hottest, blue is coldest. Use gradient

    let mut color = (0, 0, 0);
    if temperature < 0.0 {
        color = (0, 0, 255);
    } else if temperature < 100.0 {
        // blue to green
        let blue = 255 - (temperature / 100.0 * 255.0) as u8;
        let green = (temperature / 100.0 * 255.0) as u8;
        color = (0, green, blue);
    } else if temperature < 200.0 {
        // green to yellow
        let green = 255 - ((temperature - 100.0) / 100.0 * 255.0) as u8;
        let red = ((temperature - 100.0) / 100.0 * 255.0) as u8;
        color = (red, green, 0);
    } else if temperature < 300.0 {
        // yellow to red
        let red = 255 - ((temperature - 200.0) / 100.0 * 255.0) as u8;
        let green = ((temperature - 200.0) / 100.0 * 255.0) as u8;
        color = (red, green, 0);
    } else {
        color = (255, 0, 0);
    }

    color
}

pub fn load_img(path: &str) -> Image {
    let vec = std::fs::read(path).unwrap();
    // we need to resize the image to 1/4 of the original size
    Image::from_file_with_format(&vec, None).unwrap()
}

// Draw wind direction arrows on the image
fn draw_wind_arrow(image: &mut Image, x: u32, y: u32, dx: i8, dy: i8, strength: f32) {
    // Scale arrow length based on wind strength with improved visibility
    // Further increase minimum length and maximum length for better visibility
    let arrow_length = (strength * 4.0).min(12.0) as i32;

    // Skip drawing for very weak wind or no wind
    if arrow_length < 2 || (dx == 0 && dy == 0) {
        return;
    }

    // Calculate end point of arrow
    let end_x = (x as i32 + (dx as i32 * arrow_length)).clamp(0, image.width() as i32 - 1) as u32;
    let end_y = (y as i32 + (dy as i32 * arrow_length)).clamp(0, image.height() as i32 - 1) as u32;

    // Determine color based on wind direction and strength
    // Use a color that will stand out more visibly in wind mode
    let color = get_wind_arrow_color(dx, dy, strength); // Draw the main arrow line with increased thickness for better visibility
                                                        // Instead of a single line, draw multiple lines for a thicker arrow
    draw_line_on_image(image, x, y, end_x, end_y, color);

    // For arrows in higher wind strengths, draw thicker lines
    if strength > 2.5 {
        // Slightly offset parallel lines for thicker appearance
        if x + 1 < image.width() as u32 && end_x + 1 < image.width() as u32 {
            draw_line_on_image(image, x + 1, y, end_x + 1, end_y, color);
        }
        if y + 1 < image.height() as u32 && end_y + 1 < image.height() as u32 {
            draw_line_on_image(image, x, y + 1, end_x, end_y + 1, color);
        }
    }

    // Draw enhanced arrow head with better visibility
    if arrow_length > 3 {
        let head_length = arrow_length / 2; // Increase head size relative to arrow

        // Calculate perpendicular vectors for arrow head
        let dx_f = dx as f32;
        let dy_f = dy as f32;
        let length = (dx_f * dx_f + dy_f * dy_f).sqrt();

        if length > 0.0 {
            let normalized_dx = dx_f / length;
            let normalized_dy = dy_f / length;

            // Perpendicular vector
            let perp_dx = -normalized_dy;
            let perp_dy = normalized_dx;

            // First arrow head point
            let head1_x = (end_x as i32 - (normalized_dx * head_length as f32) as i32
                + (perp_dx * head_length as f32 * 0.5) as i32)
                .clamp(0, image.width() as i32 - 1) as u32;
            let head1_y = (end_y as i32 - (normalized_dy * head_length as f32) as i32
                + (perp_dy * head_length as f32 * 0.5) as i32)
                .clamp(0, image.height() as i32 - 1) as u32;

            // Second arrow head point
            let head2_x = (end_x as i32
                - (normalized_dx * head_length as f32) as i32
                - (perp_dx * head_length as f32 * 0.5) as i32)
                .clamp(0, image.width() as i32 - 1) as u32;
            let head2_y = (end_y as i32
                - (normalized_dy * head_length as f32) as i32
                - (perp_dy * head_length as f32 * 0.5) as i32)
                .clamp(0, image.height() as i32 - 1) as u32;
            // Draw the arrow head lines with the same color as the main arrow
            draw_line_on_image(image, end_x, end_y, head1_x, head1_y, color);
            draw_line_on_image(image, end_x, end_y, head2_x, head2_y, color);
        }
    }
}

// Helper function to determine wind arrow color based on direction and strength
fn get_wind_arrow_color(dx: i8, dy: i8, strength: f32) -> Color {
    // Calculate intensity based on wind strength - increased intensity for better visibility
    let intensity = ((0.7 + strength * 0.3).min(1.0) * 255.0) as u8;

    // Base opacity on strength - stronger wind is more visible - increased for better visibility
    let alpha = ((0.9 + strength * 0.1).min(1.0) * 255.0) as u8;

    // Enhanced visibility for wind arrows with improved colors
    if dx.abs() > dy.abs() {
        // Horizontal wind
        if dx > 0 {
            color_u8!(80, 200, intensity, alpha) // Right wind - bright blue
        } else {
            color_u8!(intensity, 100, 255, alpha) // Left wind - bright purple
        }
    } else {
        // Vertical wind
        if dy > 0 {
            color_u8!(50, intensity, 220, alpha) // Down wind - bright teal
        } else {
            color_u8!(intensity, intensity, 40, alpha) // Up wind - bright yellow
        }
    }
}

// Helper function to draw a line on an Image
fn draw_line_on_image(image: &mut Image, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
    // Bresenham's line algorithm
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < image.width() as i32 && y0 < image.height() as i32 {
            image.set_pixel(x0 as u32, y0 as u32, color);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            err += dx;
            y0 += sy;
        }
    }
}
