use std::fs;
use std::io;
use std::path::PathBuf;
#[derive(Debug)]
struct Arguments<'a> {
    input_extension: &'a String,
    rr_multiplier: f32,
    diff: bool,
    skip: i32,
    sampling_rate: f32,
}

fn parse_args(argv: &Vec<String>) -> Arguments {
    let owned_multiplier = &argv[2].to_string();
    let multiplier = owned_multiplier.parse::<f32>().unwrap();
    let owned_diff = &argv[3].to_string();
    let diff = owned_diff.parse::<bool>().unwrap();
    let owned_skip = &argv[4].to_string();
    let skip = owned_skip.parse::<i32>().unwrap();
    let sampling_rate: f32;
    if argv.len() == 6 {
        let owned_sampling_rate = &argv[5].to_string();
        sampling_rate = owned_sampling_rate.parse::<f32>().unwrap();
    } else {
        sampling_rate = 0.0;
    }
    Arguments {
        input_extension: &argv[1],
        rr_multiplier: multiplier,
        diff,
        skip,
        sampling_rate,
    }
}

fn read_lines(filepath: &str, args: &Arguments) -> Vec<Vec<String>> {
    let contents = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(_) => {
            println!("failed to read from file {}", &filepath);
            std::process::exit(1);
        }
    };

    let owned_lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut rr_idx: i32 = 0;
    let mut prev: f32 = 0.0;
    let mut current: f32;
    for s in &owned_lines {
        if rr_idx < args.skip {
            rr_idx += 1;
            continue;
        }

        let mut line: Vec<String> = Vec::new();
        for (i, word) in s.split_whitespace().enumerate() {
            let mut owned_word = word.to_string();
            if i == 0 {
                if let Ok(num) = owned_word.parse::<f32>() {
                    if args.diff {
                        current = num - prev;
                        prev = num;
                    } else {
                        current = num
                    }

                    if args.diff && rr_idx == args.skip {
                        prev = num;
                    }
                    if args.sampling_rate == 0.0 {
                        owned_word = (current * args.rr_multiplier).to_string();
                    } else {
                        owned_word =
                            (current * args.rr_multiplier / args.sampling_rate).to_string();
                    }
                }
            }
            if (args.diff && rr_idx > args.skip) || (!args.diff) {
                line.push(owned_word);
            }
        }
        if (args.diff && rr_idx > args.skip) || (!args.diff) {
            result.push(line);
        }
        rr_idx += 1;
    }

    result
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
            let contents = read_lines(&entry_path.to_str().unwrap(), &args);
            println!("contents length: {:?}", contents);
        }
    }
    Ok(())
}
