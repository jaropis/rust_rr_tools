use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let arg1 = std::env::args().nth(1).expect("usage: prog EXT");
    let wanted_ext = arg1.as_str();
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    println!("current dir: {:?}", current_dir);
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry_path = entry.unwrap().path();
        if entry_path.extension().and_then(|s| s.to_str()) == Some(wanted_ext) {
            println!("file: {:?}", entry_path);
        }
    }
    Ok(())
}
