use std::fs::File;
use polars::prelude::*;

fn apply_s(s: polars::prelude::Series) -> polars::prelude::Series {
    let mut collected: Vec<f64> = vec![];
    for x in s.list().iter() {
        for y in x.into_iter() {
            for z in y.unwrap().f64().into_iter() {
                collected = z.into_iter().map(|x| x.unwrap()).collect();
            }
        }
    }
    println!("{:?}", collected);
    s
}

fn main() {
    let f = File::open("coef.parquet").unwrap();
    let df_parquet = ParquetReader::new(f).finish().unwrap();
    let coef = df_parquet
        .clone()
        .lazy()
        .select([col("coef")])
        .with_columns([
            col("coef")
            .apply(|s| Ok(Some(apply_s(s))), GetOutput::default())
        ])
        .collect();
}
