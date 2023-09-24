use std::{
    io::{
        self,
        prelude::*,
        BufReader,
    },
    fs::File,
    path::PathBuf,
};

// TODO: read binary ROM files
// TODO: read plaintext ROM files (with and without additional memory writes)

// Read hexadecimal bytes from plaintext file
#[allow(dead_code)]
pub fn load_bytes(path: &PathBuf) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    Ok(
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|l| l.len() >= 2)
            .map(|l| {
                l.chars().take(2).collect::<String>()
            })
            .map(|string| {
                hex::decode(string).unwrap_or(vec![0]).first().unwrap().to_owned()
            })
            .collect()
        )
} 

pub fn load_rom(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}
