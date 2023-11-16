use macroquad::miniquad::window::{screen_size, show_keyboard};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use rayon::prelude::*;
use silica_engine::group::ElementManager;
use silica_engine::{prelude::*, world};
use std::convert::TryInto;
use std::ops::AddAssign;

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
#[derive(Clone, Copy, Debug)]
struct GameProperties {
    tool_radius: f32,
    tool_type: Variant,
    hovering_over: Variant,
    selected_group_idx: usize,
}

struct WorldInfo {
    fps: i32,
    properties: GameProperties,
    world_width: usize,
    world_height: usize,
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initialization

    let w: usize = 611;
    let h: usize = 383;
    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let mut world: World = World::new(w as i32, h as i32);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut game_properties = GameProperties {
        tool_radius: 10.0,
        tool_type: Variant::Sand,
        selected_group_idx: 0,
        hovering_over: Variant::Empty,
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
        world_info.fps = get_fps();

        world.tick();
        let w = image.width();
        let h = image.height();

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0 as usize;
        let mouse_y = mouse_pos.1 as usize;

        // convert screen coords to world coords for mouse
        let (screen_w, screen_h) = screen_size();
        let mouse_x_world = (mouse_x as f32 / screen_w * w as f32) as usize;
        let mouse_y_world = (mouse_y as f32 / screen_h * h as f32) as usize;

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
        /* */
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

        let mouse_wheel = mouse_wheel().1;
        //world_info.properties.tool_radius += mouse_wheel;
        // use logarithmic scale for radius change
        if mouse_wheel > 0.0 {
            world_info.properties.tool_radius *= 1.1;
        } else if mouse_wheel < 0.0 {
            world_info.properties.tool_radius /= 1.1;
        }
        // handle input
        if is_mouse_button_down(MouseButton::Left)
            && mouse_y < screen_h as usize - 60
            && mouse_x < screen_w as usize - 200
        {
            // use screen coords mapped to world coords
            // make sure that the particle at the mouse position is empty
            if world
                .get_particle(mouse_x_world as i32, mouse_y_world as i32)
                .variant
                == Variant::Empty
            {
                paint_radius(
                    &mut world,
                    mouse_x_world as i32,
                    mouse_y_world as i32,
                    world_info.properties.tool_type,
                    world_info.properties.tool_radius as i32,
                );
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            erase_radius(
                &mut world,
                mouse_x_world as i32,
                mouse_y_world as i32,
                world_info.properties.tool_radius as i32,
            );
        }
        for touch in touches() {
            println!("Touch at {}, {}", touch.position.x, touch.position.y);
            world.set_particle(
                touch.position.x as i32,
                touch.position.y as i32,
                Variant::Sand,
            );
        }

        // if z is pressed, draw a zoomed in version of the world at the mouse position inside a rect

        if is_key_pressed(KeyCode::R) {
            world.reset();
        }

        if is_key_pressed(KeyCode::Space) {
            world.running = !world.running;
        }

        game_properties.hovering_over = world
            .get_particle(mouse_x_world as i32, mouse_y_world as i32)
            .variant;

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

fn draw_walls(world: &mut World) {
    for x in 0..world.width {
        world.set_particle(x as i32, 0, Variant::Wall);
        world.set_particle(x as i32, world.height as i32 - 1, Variant::Wall);
    }
    for y in 0..world.height {
        world.set_particle(0, y as i32, Variant::Wall);
        world.set_particle(world.width as i32 - 1, y as i32, Variant::Wall);
    }
}

fn draw_group_sidebar(manager: &ElementManager, world_info: &mut WorldInfo) {
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
fn draw_element_list(manager: &ElementManager, world_info: &mut WorldInfo) {
    let button_size = 60.0;

    let ui = root_ui().window(
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
                let button = widgets::Button::new(element.to_string())
                    .position(vec2(x, 0.0))
                    .size(vec2(100.0, 60.0))
                    .ui(ui);

                // if button is selected, draw a rectangle around it
                if world_info.properties.tool_type == element {
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
                    world_info.properties.tool_type = element;
                }
                x += 100.0;
            }
        },
    );
}
fn draw_top_panel(world_info: &mut WorldInfo) {
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
    draw_text(
        &format!("{:?}", world_info.properties.hovering_over),
        screen_width() - 200.0,
        20.0,
        30.0,
        Color::new(1.0, 1.0, 1.0, 1.0), // Adjust the color as needed
    );
}

fn draw_tool_outline(world_info: &mut WorldInfo) {
    // get mouse pos
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_x = mouse_x as usize;
    let radius = world_info.properties.tool_radius;
    draw_circle_lines(
        mouse_x as f32,
        mouse_y as f32,
        radius,
        2.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
}

fn paint_radius(world: &mut World, x: i32, y: i32, variant: Variant, radius: i32) {
    for dx in -radius..radius {
        for dy in -radius..radius {
            if dx * dx + dy * dy < radius * radius {
                world.set_particle(x + dx, y + dy, variant);
            }
        }
    }
}

fn erase_radius(world: &mut World, x: i32, y: i32, radius: i32) {
    for dx in -radius..radius {
        for dy in -radius..radius {
            if dx * dx + dy * dy < radius * radius {
                world.set_particle(x + dx, y + dy, Variant::Empty);
            }
        }
    }
}

fn register_element_groups(manager: &ElementManager) {
    manager.register_group("Powders", vec![Variant::Sand, Variant::Salt]);
    manager.register_group("Liquids", vec![Variant::Water, Variant::SaltWater]);
    manager.register_group("Gases", vec![Variant::Smoke]);
    manager.register_group("Explosives", vec![Variant::Fire]);
}
