use std::{
    fs::File,
    io::{BufWriter, Write},
};

use dotenvy::dotenv;
use rust_backend::graphql::frontoffice::FrontOfficeSchema;

fn main() {
    dotenv().ok();
    let sdl = FrontOfficeSchema::default().sdl();
    let mut output_file = BufWriter::new(File::create("src/schemas/frontOffice.graphqls").unwrap());
    output_file.write_all(sdl.as_bytes()).unwrap();
    output_file.flush().unwrap();
}
