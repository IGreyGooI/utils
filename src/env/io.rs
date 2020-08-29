use std::{fs, path};
use std::io::Read;

pub fn load_file_as_u8<P: AsRef<path::Path>>(file_path: &P) -> Box<[u8]> {
    let mut buf = Vec::new();
    fs::File::open(file_path)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf.into_boxed_slice()
}
