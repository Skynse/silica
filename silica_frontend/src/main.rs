mod gameworld;
mod input;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::WindowResolution;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use eframe::epaint::image;
use gameworld::GameWorld;
use silica_engine::{api::API, world::World};
use silica_engine::{particle::particle_to_color, particle::Particle, variant::Variant};

static WIDTH: f32 = 700.;
static HEIGHT: f32 = 700.;

#[derive(Component)]
struct Properties {
    pub parts: i32,
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH, HEIGHT)),
        ..Default::default()
    };

    let world = World::new(WIDTH as i32, HEIGHT as i32);

    let app = App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Silica".to_string(),
                        resolution: WindowResolution::new(1024., 600.),
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Startup, setup)
        .add_systems(Update, ui_canvas)
        .add_systems(Update, update_world)
        .add_plugins(input::InputPlugin)
        .run();
}

fn ui_canvas(mut contexts: EguiContexts) {
    egui::Window::new("Silica").show(contexts.ctx_mut(), |ui| {
        ui.label("Floating pan");
    });
}

pub fn update_world(
    mut images: ResMut<Assets<Image>>,
    mut world: Query<(&mut GameWorld, &Handle<Image>)>,
) {
    let world = world.get_single_mut();
    if world.is_err() {
        return;
    }
    let (mut world, image_handle) = world.unwrap();
    if world.running {
        world.world.tick();
    }

    let image = images.get_mut(image_handle).unwrap();
    for x in 0..world.width() {
        for y in 0..world.height() {
            let idx = world.world.get_idx(x as i32, y as i32);
            let particle = world.world.get_particle(x as i32, y as i32);
            let bbp = 4;
            let idx = idx * bbp;
            let color = particle_to_color(particle.get_variant());

            image.data[idx] = color.0;
            image.data[idx + 1] = color.1;
            image.data[idx + 2] = color.2;
            image.data[idx + 3] = 255;
        }
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_world(commands, images.as_mut(), 255, 255);
}

fn spawn_world(mut commands: Commands, images: &mut Assets<Image>, width: u32, height: u32) {
    let image_handle = {
        let image = Image::new_fill(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 255, 0, 255],
            TextureFormat::Rgba8UnormSrgb,
        );
        images.add(image)
    };

    commands
        .spawn(GameWorld::new(width as f32, height as f32))
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        });
}
