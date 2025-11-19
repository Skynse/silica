// Advanced wind visualization and interaction utilities
use macroquad::prelude::*;
use silica_engine::wind::Wind;

// Get appropriate color for wind visualization based on background brightness
pub fn get_adaptive_wind_color(wind: &Wind, background_color: Color) -> Color {
    // Calculate wind strength for intensity
    let wind_strength = wind.strength();

    // Base color intensity on wind strength
    let intensity = ((0.4 + wind_strength * 0.1).min(1.0) * 255.0) as u8;

    // Calculate background brightness
    let bg_brightness =
        (background_color.r * 0.299 + background_color.g * 0.587 + background_color.b * 0.114)
            * 255.0;

    // Determine if background is dark or light
    let is_dark_bg = bg_brightness < 128.0;

    // Base opacity - stronger for dark backgrounds
    let alpha = if is_dark_bg {
        ((0.8 + wind_strength * 0.05).min(1.0) * 255.0) as u8
    } else {
        ((0.6 + wind_strength * 0.05).min(0.9) * 255.0) as u8
    };

    // Determine color based on wind direction with adaptation for background
    if wind.dx.abs() > wind.dy.abs() {
        // Horizontal wind
        if wind.dx > 0 {
            // Right wind - cyan/blue
            if is_dark_bg {
                Color::new(0.3, 0.7, intensity as f32 / 255.0, alpha as f32 / 255.0)
            } else {
                Color::new(0.0, 0.5, intensity as f32 / 255.0, alpha as f32 / 255.0)
            }
        } else {
            // Left wind - purple/magenta
            if is_dark_bg {
                Color::new(intensity as f32 / 255.0, 0.3, 0.7, alpha as f32 / 255.0)
            } else {
                Color::new(intensity as f32 / 255.0, 0.1, 0.6, alpha as f32 / 255.0)
            }
        }
    } else {
        // Vertical wind
        if wind.dy > 0 {
            // Down wind - greenish
            if is_dark_bg {
                Color::new(0.3, intensity as f32 / 255.0, 0.7, alpha as f32 / 255.0)
            } else {
                Color::new(0.0, intensity as f32 / 255.0, 0.5, alpha as f32 / 255.0)
            }
        } else {
            // Up wind - yellow/orange
            if is_dark_bg {
                Color::new(
                    intensity as f32 / 255.0,
                    intensity as f32 / 255.0,
                    0.4,
                    alpha as f32 / 255.0,
                )
            } else {
                Color::new(
                    intensity as f32 / 255.0,
                    intensity as f32 / 255.0,
                    0.0,
                    alpha as f32 / 255.0,
                )
            }
        }
    }
}

// Calculate optimized grid size for wind indicators based on resolution
pub fn calculate_wind_grid_size(width: u32, height: u32) -> u32 {
    // Adjust divisor based on resolution to maintain readable visualization
    let base_divisor = if width > 1200 || height > 1200 {
        48.0 // For very high resolutions
    } else if width > 800 || height > 800 {
        42.0 // For high resolutions
    } else if width > 500 || height > 500 {
        36.0 // For medium resolutions
    } else {
        32.0 // For low resolutions
    };

    // Calculate grid size with minimum to avoid too small grid cells
    (width.min(height) as f32 / base_divisor).round().max(4.0) as u32
}

// Apply wind effect to particle color for visualization
pub fn blend_particle_with_wind_color(particle_color: Color, wind: &Wind) -> Color {
    let wind_strength = wind.strength();

    // Skip blending for minimal wind
    if wind_strength < 0.5 {
        return particle_color;
    }

    // Calculate wind color
    let intensity = (wind_strength * 25.0).min(255.0) as u8;

    // Determine wind color based on direction
    let (r, g, b) = if wind.dx.abs() > wind.dy.abs() {
        if wind.dx > 0 {
            // Right wind -> blue
            (0, intensity / 2, intensity)
        } else {
            // Left wind -> red
            (intensity, 0, intensity / 2)
        }
    } else if wind.dy != 0 {
        if wind.dy > 0 {
            // Down wind -> cyan
            (0, intensity, intensity)
        } else {
            // Up wind -> yellow
            (intensity, intensity, 0)
        }
    } else {
        // No wind
        (0, 0, 0)
    };

    // Adaptive wind visibility based on strength - stronger wind shows through more
    let wind_factor = (wind_strength / 8.0).min(0.6);

    // Blend colors
    let blended_r = (1.0 - wind_factor) * particle_color.r + wind_factor * r as f32 / 255.0;
    let blended_g = (1.0 - wind_factor) * particle_color.g + wind_factor * g as f32 / 255.0;
    let blended_b = (1.0 - wind_factor) * particle_color.b + wind_factor * b as f32 / 255.0;

    Color::new(blended_r, blended_g, blended_b, particle_color.a)
}
