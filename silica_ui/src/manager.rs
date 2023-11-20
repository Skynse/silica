use std::fmt::Display;

use silica_engine::variant::{Particle, Variant};

#[derive(Clone, Copy, Debug)]
pub struct GameProperties {
    pub tool_radius: f32,
    pub tool_type: Tool,
    pub hovering_over: Particle,
    pub hovering_temperature: f32,
    pub selected_group_idx: usize,

    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Property {
    Temperature,
    COOL,
    Pressure,
}

#[derive(Clone, Copy, Debug)]
pub enum Tool {
    ElementTool(Variant),
    PropertyTool(Property),
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Temperature => write!(f, "HEAT"),
            Property::COOL => write!(f, "COOL"),
            Property::Pressure => write!(f, "PRSR"),
        }
    }
}

impl Tool {
    pub fn get_property(&self) -> Option<Property> {
        match self {
            Tool::ElementTool(_variant) => None,
            Tool::PropertyTool(property) => Some(*property),
        }
    }

    pub fn get_variant(&self) -> Option<Variant> {
        match self {
            Tool::ElementTool(variant) => Some(*variant),
            Tool::PropertyTool(_property) => None,
        }
    }
}

pub struct WorldInfo {
    pub fps: i32,
    pub properties: GameProperties,
    pub world_width: usize,
    pub world_height: usize,
    pub parts: usize,
}
