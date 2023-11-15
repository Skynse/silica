use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use silica_engine::prelude::*;
use std::convert::TryInto;

fn window_conf() -> Conf {
    Conf {
        window_title: "Silica".to_owned(),
        window_width: 1280,
        window_height: 920,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let scale_fac = 1.5;
    let w: usize = 611;
    let h: usize = 383;
    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let mut world: World = World::new(w.clone(), h.clone());
    let texture = Texture2D::from_image(&image);

    draw_walls(&mut world);

    loop {
        clear_background(BLACK);
        world.tick();
        let w = image.width();
        let h = image.height();

        for x in 0..w as u32 {
            for y in 0..h as u32 {
                let particle = world.get_particle(x as i32, y as i32);
                let color = particle_to_color(particle.variant);
                let c = color_u8!(color.0, color.1, color.2, 255);
                image.set_pixel(x as u32, y as u32, c);
                image.set_pixel(x.try_into().unwrap(), y.try_into().unwrap(), c);
                image.set_pixel(x, y, c);
            }
        }

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0 as usize;
        let mouse_y = mouse_pos.1 as usize;

        // convert screen coords to world coords for mouse
        let (screen_w, screen_h) = screen_size();
        let mouse_x_world = (mouse_x as f32 / screen_w * w as f32) as usize;
        let mouse_y_world = (mouse_y as f32 / screen_h * h as f32) as usize;

        if is_mouse_button_down(MouseButton::Left) {
            // use screen coords mapped to world coords
            world.set_particle(mouse_x_world as i32, mouse_y_world as i32, Variant::Sand);
        }
        for touch in touches() {
            println!("Touch at {}, {}", touch.position.x, touch.position.y);
            world.set_particle(
                touch.position.x as i32,
                touch.position.y as i32,
                Variant::Sand,
            );
        }
        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                source: Some(Rect::new(0.0, 0.0, w as f32, h as f32)),
                ..Default::default()
            },
        );

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
