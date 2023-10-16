use std::path::PathBuf;

pub fn get_filepath_from_args() -> PathBuf {
    std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file path as first argument")
        .into()
}
