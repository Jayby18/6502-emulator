use std::{
    io::{
        self,
        prelude::*,
        BufReader,
    },
    fs::File,
    path::PathBuf,
};

// TODO: read binary ROM files (use io::Seek?)
// TODO: read plaintext ROM files (with and without additional memory writes)

// Read hexadecimal bytes from plaintext file
pub fn load_bytes(path: &PathBuf) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    Ok(
        reader
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| l.len() >= 2)
            .map(|l| {
                l.chars().take(2).collect::<String>()
            })
            .map(|string| {
                hex::decode(string).unwrap_or(vec![0]).iter().next().unwrap().to_owned()
            })
            .collect()
        )
} 

