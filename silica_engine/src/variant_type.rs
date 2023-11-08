use crate::variant::Variant;
pub const VARIANT_COUNT: usize = 3;
pub struct VariantType {
    pub weight: u8,
    pub color: (u8, u8, u8),
    pub color2: (u8, u8, u8),
    pub source_variant: Variant,
}

pub static VARIANTS: [VariantType; VARIANT_COUNT] = [
    VariantType {
        weight: 0,
        color: (0, 0, 0),
        color2: (0, 0, 0),
        source_variant: Variant::Empty,
    },
    VariantType {
        weight: 0,
        color: (0x7F, 0x7F, 0x7F),
        color2: (0x7F, 0x7F, 0x7F),
        source_variant: Variant::Wall,
    },
    VariantType {
        weight: 0,
        color: (0xFF, 0xFF, 0x00),
        color2: (0xFF, 0xFF, 0x00),
        source_variant: Variant::Sand,
    },
];
