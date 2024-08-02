use colors_transform::{Color, Hsl};
use plotly::color::Rgb;

pub(crate) fn generate_palette(n: usize) -> Vec<Rgb> {
    let mut palette = Vec::new();
    let delta_hue = 360.0 / n as f32;
    let mut hue = 0.0;

    while palette.len() < n {
        let casted = Hsl::from(hue, 80.0, 50.0).to_rgb();
        let color = Rgb::new(casted.get_red() as u8, casted.get_green() as u8, casted.get_blue() as u8);
        palette.push(color);
        hue += delta_hue;
    }

    palette
}