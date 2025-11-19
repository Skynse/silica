use crate::manager::RenderMode;
use macroquad::prelude::Color;
use silica_engine::wind::Wind;

// Get appropriate color for wind visualization based on background brightness
pub fn get_adaptive_wind_color(wind: &Wind, render_mode: RenderMode) -> Color {
    // Calculate wind strength for intensity
    let wind_strength = wind.strength();

    // Base color intensity on wind strength
    let base_intensity = 0.4 + wind_strength * 0.15;
    let intensity = (base_intensity.min(1.0) * 255.0) as u8;

    // Determine if we're in wind mode for higher visibility
    let is_wind_mode = render_mode == RenderMode::Wind;

    // Base alpha on strength and render mode
    let alpha = if is_wind_mode {
        // Higher opacity in wind visualization mode
        ((0.8 + wind_strength * 0.1).min(1.0) * 255.0) as u8
    } else {
        ((0.6 + wind_strength * 0.05).min(0.9) * 255.0) as u8
    };

    // Determine color based on wind direction with enhanced visibility in wind mode
    if wind.dx.abs() > wind.dy.abs() {
        // Horizontal wind
        if wind.dx > 0 {
            // Right wind - cyan/blue
            if is_wind_mode {
                Color::new(0.1, 0.7, intensity as f32 / 255.0, alpha as f32 / 255.0)
            } else {
                Color::new(0.0, 0.5, intensity as f32 / 255.0, alpha as f32 / 255.0)
            }
        } else {
            // Left wind - purple/magenta
            if is_wind_mode {
                Color::new(intensity as f32 / 255.0, 0.2, 0.8, alpha as f32 / 255.0)
            } else {
                Color::new(intensity as f32 / 255.0, 0.1, 0.6, alpha as f32 / 255.0)
            }
        }
    } else {
        // Vertical wind
        if wind.dy > 0 {
            // Down wind - greenish
            if is_wind_mode {
                Color::new(0.2, intensity as f32 / 255.0, 0.8, alpha as f32 / 255.0)
            } else {
                Color::new(0.1, intensity as f32 / 255.0, 0.6, alpha as f32 / 255.0)
            }
        } else {
            // Up wind - yellow/orange
            if is_wind_mode {
                Color::new(
                    intensity as f32 / 255.0,
                    intensity as f32 / 255.0,
                    0.3,
                    alpha as f32 / 255.0,
                )
            } else {
                Color::new(
                    intensity as f32 / 255.0,
                    intensity as f32 / 255.0,
                    0.1,
                    alpha as f32 / 255.0,
                )
            }
        }
    }
}

// Calculate optimized grid size for wind indicators based on resolution
pub fn calculate_optimal_wind_grid(width: u32, height: u32) -> u32 {
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
