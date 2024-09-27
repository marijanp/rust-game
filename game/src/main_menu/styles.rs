pub mod color {
    use bevy::prelude::*;
    pub const PRIMARY: Color = Color::linear_rgb(52.0 / 255.0, 152.0 / 255.0, 219.0 / 255.0);
    pub const PRIMARY_CONTENT: Color = Color::WHITE;
    pub const PRIMARY_HOVER: Color = Color::linear_rgb(41.0 / 255.0, 128.0 / 255.0, 185.0 / 255.0);
    pub const SECONDARY: Color = Color::linear_rgb(231.0 / 255.0, 76.0 / 255.0, 60.0 / 255.0);
    pub const SECONDARY_CONTENT: Color = Color::linear_rgb(1., 1., 1.);
    pub const SECONDARY_HOVER: Color = Color::linear_rgb(192.0 / 255.0, 57.0 / 255.0, 43.0 / 255.0);
}
