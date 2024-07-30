//! # Custom plotting library
//! This is a custom plotting library for Rust powered by [Plotly.js](https://plot.ly/javascript/).
//! It tries to provide a high-level API for creating plots by imitating SNS and Matplotlib common
//! plots

use std::collections::HashMap;
use plotly::color::NamedColor;
use plotly::common::Marker;
use plotly::Scatter;
use polars::frame::DataFrame;
use polars::prelude::DataType::Float64;
use color_caster::COLOR_NAMES;
mod color_caster;

pub fn cast_column_to_numeric(data: &DataFrame, column_name: &str) -> Vec<f64> {
    let column = data.column(column_name).expect("column not found");
    column.cast(&Float64).unwrap().f64().unwrap().into_iter().map(|v| v.unwrap()).collect()
}
// TO DO: Add the column
pub fn relational_plot(x_name: &str, y_name: &str, hue: &str, data: &DataFrame) -> Box<Scatter<f64, f64>> {

    let z = data.column(hue).expect("hue column not found");

    let categories = z.unique().unwrap();
    let categories_colors = categories.str().unwrap()
        .iter().enumerate().map(|(i, v)| (v.unwrap(), COLOR_NAMES[i]))
        .collect::<HashMap<&str, NamedColor>>();

    let z = z.str()
        .unwrap().into_iter()
        .map(|v| categories_colors[v.unwrap()]).collect::<Vec<NamedColor>>();

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