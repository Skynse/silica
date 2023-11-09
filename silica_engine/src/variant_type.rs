use crate::variant::Variant;
pub const VARIANT_COUNT: usize = 5;

#[derive(PartialEq, Eq)]
pub struct VariantType {
    pub weight: u8,
    pub color: (u8, u8, u8),
    pub source_variant: Variant,
    pub variant_property: VariantProperty,
}

#[derive(PartialEq, Eq)]
pub enum VariantProperty {
    UnErasable,
    Indestructible,
    Solid,
    Powder,
    Liquid,
    Gas,
}

#[inline(always)]
pub fn variant_type(variant: Variant) -> &'static VariantType {
    &VARIANTS[variant as usize]
}

pub static VARIANTS: [VariantType; VARIANT_COUNT] = [
    // 0 Empty
    VariantType {
        weight: 0,
        color: (0, 0, 0),
        source_variant: Variant::Empty,
        variant_property: VariantProperty::Indestructible,
    },
    // 1 Wall
    VariantType {
        weight: 0,
        color: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Wall,
        variant_property: VariantProperty::UnErasable,
    },
    // 2 Sand
    VariantType {
        weight: 0,
        color: (0xFF, 0xFF, 0x00),
        source_variant: Variant::Sand,
        variant_property: VariantProperty::Powder,
    },
    // 3 Water
    VariantType {
        weight: 0,
        color: (0x00, 0x00, 0xFF),
        source_variant: Variant::Water,
        variant_property: VariantProperty::Liquid,
    },
    // 4 Fire
    VariantType {
        weight: 0,
        color: (0xFF, 0x00, 0x00),
        source_variant: Variant::Fire,
        variant_property: VariantProperty::Gas,
    },
];
