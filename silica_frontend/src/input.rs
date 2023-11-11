use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseWheel},
    },
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui::Key, EguiContexts};
use silica_engine::{variant_type::VARIANTS, world};

use crate::{gameworld::GameWorld, tools::Tools};

pub struct InputPlugin;
#[derive(Default, Resource)]
pub struct InputState {
    pub left_down: bool,
    pub right_down: bool,
    pub ctrl_down: bool,
    pub position: Vec2,
    pub paused: bool,
    pub middle_down: bool,
    pub world_position: Vec2,
    pub drag_movement: Vec2,
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
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut world: Query<&mut GameWorld>,
    mut tools: ResMut<Tools>,
    mut egui_context: EguiContexts,
    mut camera: Query<(&Camera, &mut Transform, &GlobalTransform)>,
) {
    for event in mouse_button_input_events.read() {
        match event.button {
            MouseButton::Left => mouse.left_down = event.state.is_pressed(),
            MouseButton::Right => mouse.right_down = event.state.is_pressed(),
            MouseButton::Middle => mouse.middle_down = event.state.is_pressed(),
            _ => (),
        }
    }

    for event in keyboard_input_events.read() {
        match event.key_code {
            Some(KeyCode::ControlLeft | KeyCode::ControlRight) => {
                mouse.ctrl_down = event.state.is_pressed()
            }

            Some(KeyCode::Space) => {
                if event.state.is_pressed() {
                    mouse.paused = !mouse.paused;
                }
            }
            _ => (),
        }
    }

    // Record latest position
    let last_position = mouse.position;
    for event in cursor_moved_events.read() {
        mouse.position = event.position;
    }
    mouse.drag_movement = if mouse.middle_down {
        mouse.position - last_position
    } else {
        Vec2::ZERO
    };

    let ctx = egui_context.ctx_mut();
    if ctx.wants_pointer_input() || ctx.is_pointer_over_area() || ctx.is_using_pointer() {
        mouse.left_down = false;
        mouse.right_down = false;
        return;
    }
    let mut wheel_y = 0.0;

    for event in mouse_wheel_events.read() {
        wheel_y += event.y;
    }

    let world = world.get_single_mut();

    if world.is_err() {
        return;
    }
    let mut world = world.unwrap();
    world.running = !mouse.paused;

    let (camera, mut transform, global_transform) = camera.single_mut();
    let world_pos = camera
        .viewport_to_world(global_transform, mouse.position)
        .unwrap_or(Ray {
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
        })
        .origin;

    // Zoom camera using mouse wheel
    if wheel_y > 0.0 && mouse.ctrl_down {
        transform.scale.x = (transform.scale.x * 0.9).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 0.9).clamp(0.1, 1.0);
    } else if wheel_y < 0.0 {
        transform.scale.x = (transform.scale.x * 1.1).clamp(0.1, 1.0);
        transform.scale.y = (transform.scale.y * 1.1).clamp(0.1, 1.0);
    }

    let half_width = (world.width() / 2) as f32;
    let half_height = (world.height() / 2) as f32;
    if mouse.middle_down {
        transform.translation.x -= mouse.drag_movement.x * transform.scale.x;
        transform.translation.y = mouse.drag_movement.y * transform.scale.y;

        transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
        transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
    }

    mouse.world_position = Vec2::new(
        world_pos.x + (world.width() / 2) as f32,
        (world.height() / 2) as f32 - world_pos.y,
    )
    .into();
    let x = mouse.world_position.x;
    let y = mouse.world_position.y;
    if x > 0.0 && x < world.width() as f32 && y > 0.0 && y < world.height() as f32 {
        if mouse.right_down {
            // remove particles in a circle around the mouse
            tools.paint_specific_variant(
                &mut world,
                x.floor() as usize,
                y.floor() as usize,
                silica_engine::variant::Variant::Empty,
            )
        }
        if mouse.left_down {
            // add particles in a circle around the mouse
            tools.paint(&mut world, x.floor() as usize, y.floor() as usize);
        }
    }
}
