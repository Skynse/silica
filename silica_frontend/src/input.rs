use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;
use silica_engine::world;

use crate::gameworld::GameWorld;

pub struct InputPlugin;
#[derive(Default, Resource)]
pub struct InputState {
    pub left_down: bool,
    pub right_down: bool,
    pub position: Vec2,
    pub world_position: Vec2,
}
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(PreUpdate, mouse_button_input);
    }
}

fn mouse_button_input(
    mut mouse: ResMut<InputState>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut world: Query<&mut GameWorld>,
    mut egui_context: EguiContexts,
    mut camera: Query<(&Camera, &mut Transform, &GlobalTransform)>,
) {
    for event in mouse_button_input_events.read() {
        match event.button {
            MouseButton::Left => mouse.left_down = event.state.is_pressed(),
            MouseButton::Right => mouse.right_down = event.state.is_pressed(),
            _ => (),
        }
    }

    // Record latest position
    let last_position = mouse.position;
    for event in cursor_moved_events.read() {
        mouse.position = event.position;
    }

    let ctx = egui_context.ctx_mut();
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() || ctx.is_using_pointer() {
        mouse.left_down = false;
        mouse.right_down = false;
        return;
    }

    let world = world.get_single_mut();

    if world.is_err() {
        return;
    }
    let mut world = world.unwrap();

    let (camera, mut transform, global_transform) = camera.single_mut();
    let world_pos = camera
        .viewport_to_world(global_transform, mouse.position)
        .unwrap()
        .origin;

    mouse.world_position = Vec2::new(
        world_pos.x + (world.width() / 2) as f32,
        (world.height() / 2) as f32 - world_pos.y,
    )
    .into();
    let x = mouse.world_position.x;
    let y = mouse.world_position.y;
    if x > 0.0 && x < world.width() as f32 && y > 0.0 && y < world.height() as f32 {
        if mouse.left_down {
            world.world.set_particle(
                x.floor() as i32,
                y.floor() as i32,
                silica_engine::variant::Variant::Sand,
            );
        }
        if mouse.right_down {
            world.world.set_particle(
                x.floor() as i32,
                y.floor() as i32,
                silica_engine::variant::Variant::Empty,
            );
        }
    }
}
