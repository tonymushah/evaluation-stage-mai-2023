use dsync::GenerationConfig;
use std::path::PathBuf;

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    dsync::generate_files(
        PathBuf::from_iter([dir, "src/schema.rs"]),
        PathBuf::from_iter([dir, "src/models"]),
        GenerationConfig {
            table_options: Default::default(),
            default_table_options: Default::default(),
            connection_type: String::from(
                "diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>",
            ),
            schema_path: PathBuf::from_iter([dir, "src/schema.rs"])
                .to_str()
                .unwrap()
                .into(),
            model_path: PathBuf::from_iter([dir, "src/models"])
                .to_str()
                .unwrap()
                .into(),
        },
    );
}
