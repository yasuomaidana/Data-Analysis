//! # Custom plotting library
//! This is a custom plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
//! It tries to provide a high-level API for creating plots by imitating SNS and Matplotlib common
//! plots

use std::collections::HashMap;
use colors_transform::{Color as Color2, Hsl};
use plotly::color::Rgb;
use plotly::common::Marker;
use plotly::Scatter;
use polars::frame::DataFrame;
use polars::prelude::DataType::Float64;
mod color_caster;

pub fn cast_column_to_numeric(data: &DataFrame, column_name: &str) -> Vec<f64> {
    let column = data.column(column_name).expect("column not found");
    column.cast(&Float64).unwrap().f64().unwrap().into_iter().map(|v| v.unwrap()).collect()
}

fn generate_palette(n: usize) -> Vec<Rgb> {
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
// TO DO: Add the column
pub fn relational_plot(x_name: &str, y_name: &str, hue: &str, data: &DataFrame) -> Box<Scatter<f64, f64>> {
    let z = data.column(hue).expect("hue column not found");

    let categories = z.unique().unwrap();
    let colors = generate_palette(categories.len());

    let categories_colors = categories.str().unwrap()
        .iter().enumerate().map(|(i, v)| (v.unwrap(), colors[i]))
        .collect::<HashMap<&str, Rgb>>();

    let z = z.str()
        .unwrap().into_iter()
        .map(|v| categories_colors[v.unwrap()]).collect::<Vec<Rgb>>();

    Scatter::new(cast_column_to_numeric(data, x_name), cast_column_to_numeric(data, y_name))
        .mode(plotly::common::Mode::Markers)
        .marker(Marker::new().color_array(z))
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_relational_plot() {
        let s0 = Series::new("a", [1, 2, 3].as_ref());
        let s1 = Series::new("b", [1, 2, 3].as_ref());
        let s2 = Series::new("c", ["M", "M", "B"].as_ref());
        let df = DataFrame::new(vec![s0, s1, s2]).unwrap();

        relational_plot("a", "b", "c", &df);
    }
}