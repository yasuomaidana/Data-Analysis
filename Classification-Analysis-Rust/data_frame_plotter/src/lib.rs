//! # Custom plotting library
//! This is a custom plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
//! It tries to provide a high-level API for creating plots by imitating SNS and Matplotlib common
//! plots

use plotly::color::Rgb;
use plotly::common::Marker;
use plotly::Scatter;
use std::collections::HashMap;
// use plotly::{Layout, Scatter};
// use plotly::layout::GridPattern::Independent;
// use plotly::layout::LayoutGrid;
use polars::frame::DataFrame;
use polars::prelude::DataType::Float64;
use polars::prelude::{col, lit, IntoLazy};

mod color_caster;
use crate::color_caster::generate_color_map;

pub fn cast_column_to_numeric(data: &DataFrame, column_name: &str) -> Vec<f64> {
    let column = data.column(column_name).expect("column not found");
    column
        .cast(&Float64)
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect()
}

pub fn single_relational_plot_with_colors(
    x_name: &str,
    y_name: &str,
    hue: &str,
    data: &DataFrame,
    hue_colors: &HashMap<String, Rgb>,
) -> Box<Scatter<f64, f64>> {
    let z = data
        .column(hue)
        .expect("hue column not found")
        .str()
        .unwrap()
        .into_iter()
        .map(|v| hue_colors[v.unwrap()])
        .collect::<Vec<Rgb>>();

    Scatter::new(
        cast_column_to_numeric(data, x_name),
        cast_column_to_numeric(data, y_name),
    )
    .mode(plotly::common::Mode::Markers)
    .marker(Marker::new().color_array(z))
}

// TO DO: Add the column
pub fn single_relational_plot(
    x_name: &str,
    y_name: &str,
    hue: &str,
    data: &DataFrame,
) -> Vec<Box<Scatter<f64, f64>>> {
    let categories_colors = generate_color_map(hue, data);

    categories_colors
        .keys()
        .map(|category| {
            let category_data = data
                .clone()
                .lazy()
                .filter(col(hue).eq(lit(category.as_str())))
                .collect()
                .unwrap();
            let trace = single_relational_plot_with_colors(
                x_name,
                y_name,
                hue,
                &category_data,
                &categories_colors,
            );
            trace.name(category)
        })
        .collect()
}

// pub fn relational_plot(x_name: &str, y_name: &str, hue: &str, columns: Vec<&str>, data: &DataFrame) {
//
//     let rows = (columns.len() as f64).sqrt().ceil() as usize;
//     let layout = Layout::new()
//         .grid(LayoutGrid::new().rows(rows).columns(rows).pattern(Independent))
//         .height(columns.len()*200);
//
//     let mut plot = plotly::Plot::new();
//     for i in 0..columns.len() {
//         let x_axis = match i { 0 => "x", _ => &*format!("x{}", i) };
//         let y_axis = match i { 0 => "y",  _ => &*format!("y{}", i)};
//
//         let trace = single_relational_plot(x_name, y_name, hue, data)
//             .x_axis(x_axis)
//             .y_axis(y_axis);
//         plot.add_trace(trace);
//     }
//     plot.show();
// }

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

        single_relational_plot("a", "b", "c", &df);
    }
}
