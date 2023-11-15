use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use silica_engine::prelude::*;
use std::convert::TryInto;

#[macroquad::main("BasicShapes")]
async fn main() {
    let w: usize = screen_width() as usize;
    let h: usize = screen_height() as usize;
    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let mut world: World = World::new(w.clone(), h.clone());
    let window_width = screen_width();
    let window_height = screen_height();

    let scale_x = w as f32 / window_width;
    let scale_y = h as f32 / window_height;
    let texture = Texture2D::from_image(&image);

    draw_walls(&mut world);

    loop {
        clear_background(BLACK);
        world.tick();
        let w = image.width();
        let h = image.height();

        for x in 0..w as u32 / 2 {
            for y in 0..h as u32 / 2 {
                let particle = world.get_particle(x as i32, y as i32);
                let color = particle_to_color(particle.variant);
                let c = color_u8!(color.0, color.1, color.2, 255);
                image.set_pixel(x as u32, y as u32, c);
                image.set_pixel(x.try_into().unwrap(), y.try_into().unwrap(), c);
                image.set_pixel(x, y, c);
            }
        }

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            world.set_particle(
                (mouse_x * scale_x) as i32,
                (mouse_y * scale_y) as i32,
                Variant::Sand,
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
        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height() * scale_y)),
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
