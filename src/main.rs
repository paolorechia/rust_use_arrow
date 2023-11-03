use std::fs::File;
use onnxruntime::environment::Environment;
use onnxruntime::LoggingLevel;
// use polars::prelude::*;


fn main() {
    let f = File::open("data/predict.parquet").unwrap();
    let environment = Environment::builder()
    .with_name("test")
    .with_log_level(LoggingLevel::Verbose)
    .build().unwrap();

    let mut session = environment
        .new_session_builder().unwrap()
        .with_number_threads(1).unwrap()
        .with_model_from_file("data/linear.onnx").unwrap();

    println!("Model loaded!");
}
