use std::fs;
use std::io;
use std::path::PathBuf;
#[derive(Debug)]
struct Arguments<'a> {
    input_extension: &'a String,
}

fn parse_args(argv: &Vec<String>) -> Arguments {
    Arguments {
        input_extension: &argv[1],
    }
}

fn read_lines(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(_) => {
            println!("failed to read from file {}", &filepath);
            std::process::exit(1);
        }
    }
}

fn main() -> io::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = parse_args(&argv);
    println!("args: {:?}", args);
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    println!("current dir: {:?}", current_dir);
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry_path = entry.unwrap().path();
        if entry_path.extension().and_then(|s| s.to_str()) == Some(args.input_extension) {
            println!("file: {:?}", entry_path);
            let contents = read_lines(&entry_path.to_str().unwrap());
            println!("contents: {}", contents);
        }
    }
    Ok(())
}
