use bevy::{prelude::*, render::camera, transform::commands};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use silica_engine::variant::Variant;

use crate::{gameworld::GameWorld, tools::Tools};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gui)
            .add_systems(Update, side_panel);
    }
}
#[derive(Resource)]
pub struct GUI {}

pub fn setup_gui(mut commands: Commands, mut contexts: EguiContexts) {
    let mut style = egui::Style::default();
    commands.insert_resource(GUI {})
}

pub struct VariantOption {
    variant: Variant,
    name: &'static str,
}

static VARIANT_OPTIONS: [VariantOption; 4] = [
    VariantOption {
        variant: Variant::Sand,
        name: "Sand",
    },
    VariantOption {
        variant: Variant::Water,
        name: "Water",
    },
    VariantOption {
        variant: Variant::Wall,
        name: "Wall",
    },
    VariantOption {
        variant: Variant::Fire,
        name: "Fire",
    },
];

pub fn side_panel(
    mut contexts: EguiContexts,
    mut world: Query<&mut GameWorld>,
    mut gui: ResMut<GUI>,
    mut tools: ResMut<Tools>,
    mut camera: Query<&mut Transform, With<Camera>>,
    images: ResMut<Assets<Image>>,
) {
    let ctx = contexts.ctx_mut();
    let mut world = world.single_mut();
    let left: f32 = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Properties");

            ui.separator();
            // button to clear the world
            if ui.button("Clear world").clicked() {
                world.reset();
            }

            ui.separator();

            // variant selectors
            ui.label("Variant");
            ui.separator();
            for variant_option in VARIANT_OPTIONS.iter() {
                if ui
                    .radio(
                        tools.variant == variant_option.variant,
                        variant_option.variant.to_string(),
                    )
                    .clicked()
                {
                    tools.variant = variant_option.variant;
                }
            }
        })
        .response
        .rect
        .width();
}

pub fn variant_button(ui: &mut Ui, gui: &mut GUI, variant: Variant) {
    let text_button = ui.button(variant.to_string());
}
