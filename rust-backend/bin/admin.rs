use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dotenvy::dotenv;
use rust_backend::graphql::admin::AdminSchema;

fn main() {
    dotenv().ok();
    let sdl = AdminSchema::default().sdl();
    let mut output_file = BufWriter::new(File::create("src/schemas/admin.graphqls").unwrap());
    output_file.write_all(sdl.as_bytes()).unwrap();
    output_file.flush().unwrap();
}
