use std::collections::HashMap;
use polars::export::rayon::iter::IntoParallelRefIterator;
use polars::frame::DataFrame;

pub struct Encoder {
    encoder: HashMap<(String, String), usize>,
    decoder: HashMap<(String, usize), String>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            encoder: HashMap::new(),
            decoder: HashMap::new(),
        }
    }
    pub fn fit_transform(&mut self, df: &DataFrame) -> DataFrame {
        let mut encoder = &mut self.encoder;
        let mut decoder = &mut self.decoder;
        df.par_iter()
            .iter().for_each(|series| {
                let name = series.name();
                let mut counter = 0;
                series.map_in_place(|value| {
                    let key = (name.clone(), value.clone());
                    if !encoder.contains_key(&key) {
                        encoder.insert(key.clone(), counter);
                        decoder.insert((name.clone(), counter), value.clone());
                        counter += 1;
                    }
                    encoder.get(&key).unwrap().clone()
                });
            });
    }
    
    pub fn decode(&self, df: &DataFrame) -> DataFrame {
        df.select(df.get_columns())
            .unwrap()
            .par_iter()
            .iter()
            .map(|series| {
                let name = series.name();
                series.map(|value| self.decoder.get(&(name.clone(), value)).unwrap())
            })
    }
}
