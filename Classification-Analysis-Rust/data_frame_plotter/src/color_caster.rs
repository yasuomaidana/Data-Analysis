use std::collections::HashMap;
use colors_transform::{Color, Hsl};
use plotly::color::Rgb;
use polars::frame::DataFrame;

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

pub(crate) fn generate_color_map(column:&str, data: &DataFrame) -> HashMap<String, Rgb> {
    let z = data.column(column).expect("hue column not found");
    let categories = z.unique().unwrap();
    let colors = generate_palette(categories.len());

    categories.str().unwrap()
        .iter().enumerate().map(|(i, v)| (String::from(v.unwrap()), colors[i]))
        .collect::<HashMap<String, Rgb>>()
}