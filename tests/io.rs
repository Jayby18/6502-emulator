use mos_6502_emulator::io;
use dirs;
use std::path::{
    Path,
    PathBuf,
};

#[test]
fn read_existing() {
    // TODO: write to a new file before testing, this one could change
    let path = dirs::home_dir().unwrap().join(PathBuf::from("rom.txt"));
    let result = io::load_bytes(&path).unwrap();
    assert_eq!(result, vec![0xA9, 0, 0, 0xE3, 0xF1]);
}

#[test]
fn read_nonexisting() {
    let path = dirs::home_dir().unwrap().join(PathBuf::from("idontexist.txt"));
    io::load_bytes(&path).expect_err("File does exist, somehow.");
}
