mod data;
mod manager;
mod utils;

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

const TOOLS: [Property; 4] = [
    Property::Temperature,
    Property::COOL,
    Property::Pressure,
    Property::DelWall,
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
        hovering_over: EMPTY_CELL,
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
        } else {
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
    manager.register_group("Life", vec![Variant::GOL])
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
