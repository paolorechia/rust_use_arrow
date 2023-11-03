use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Field;
use std::{fs::File, path::Path};

fn read_parquet() {
    let paths = vec![
        "coef.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        for col in row.unwrap().get_column_iter() {
            let w: Field = (*col.1).clone().into();
            // println!("Type name: {:?}", type_name);
            match w {
                Field::ListInternal(list) => {
                    println!("Found ListInternal: {:?}", list.elements());
                },
                _ => {
                    println!("Unrecognized type");
                }
            };
        }
    }
}