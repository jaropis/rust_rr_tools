use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct Arguments<'a> {
    input_extension: &'a String,
    output_extension: &'a String,
    rr_multiplier: f32,
    diff: bool,
    skip: i32,
    sampling_rate: f32,
    scout: bool,
    correct: bool,
}

fn form_result_path(filepath: &str, extension: &str) -> String {
    let result_path = std::path::Path::new(filepath);
    let stem = result_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let dir = result_path.parent().and_then(|s| s.to_str()).unwrap_or("");
    format!("{}/{}.{}", dir, stem, extension)
}
fn parse_args(argv: &Vec<String>) -> Arguments {
    let owned_multiplier = &argv[3].to_string();
    let multiplier = owned_multiplier.parse::<f32>().unwrap();
    let owned_diff = &argv[4].to_string();
    let diff = owned_diff.parse::<bool>().unwrap();
    let owned_skip = &argv[5].to_string();
    let skip = owned_skip.parse::<i32>().unwrap();
    let sampling_rate: f32;
    let owned_scout = &argv[6].to_string();
    let scout = owned_scout.parse::<bool>().unwrap();
    let owned_correct = &argv[7].to_string();
    let correct = owned_correct.parse::<bool>().unwrap();
    if argv.len() == 9 {
        let owned_sampling_rate = &argv[8].to_string();
        sampling_rate = owned_sampling_rate.parse::<f32>().unwrap();
    } else {
        sampling_rate = 0.0;
    }
    Arguments {
        input_extension: &argv[1],
        output_extension: &argv[2],
        rr_multiplier: multiplier,
        diff,
        skip,
        sampling_rate,
        scout,
        correct,
    }
}

fn correct_processed_rr_intervals(data: &mut Vec<Vec<String>>) {
    println!("Starting correction with {} rows", data.len());

    // calculating average of all RR intervals with flag 0
    let mut sum_flag_0: f32 = 0.0;
    let mut count_flag_0: i32 = 0;

    for row in data.iter() {
        if row.len() >= 2 && row[1] == "0" {
            if let Ok(rr_value) = row[0].parse::<f32>() {
                sum_flag_0 += rr_value;
                count_flag_0 += 1;
            }
        }
    }

    if count_flag_0 == 0 {
        println!("Warning: No RR intervals with flag 0 found for correction");
        return;
    }

    let average_rr_flag_0 = sum_flag_0 / count_flag_0 as f32;
    let threshold = 5.0 * average_rr_flag_0;

    println!(
        "Average RR for flag 0: {:.3}, threshold: {:.3}",
        average_rr_flag_0, threshold
    );

    // replacing outliers with the average
    let mut corrected_count = 0;
    for row in data.iter_mut() {
        if row.len() >= 2 {
            if let Ok(rr_value) = row[0].parse::<f32>() {
                if rr_value.abs() > threshold {
                    // Use abs() to catch negative outliers too
                    println!("Correcting RR value {} to {}", rr_value, average_rr_flag_0);
                    row[0] = average_rr_flag_0.to_string();
                    corrected_count += 1;
                }
            }
        }
    }

    println!("Corrected {} outlier RR intervals", corrected_count);
}

fn read_lines(filepath: &str, args: &Arguments) -> (Vec<Vec<String>>, HashSet<String>) {
    let contents = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(_) => {
            println!("failed to read from file {}", &filepath);
            std::process::exit(1);
        }
    };

    let owned_lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let mut result: Vec<Vec<String>> = Vec::new();
    let mut annotations: HashSet<String> = HashSet::new();
    let mut rr_idx: i32 = 0;
    let mut prev: f32 = 0.0;
    let mut prev_annot = "0".to_string();
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
            } else if i == 1 {
                owned_word = match owned_word.as_str() {
                    "N" => "0".to_string(),
                    "V" => "1".to_string(),
                    "S" => "2".to_string(),
                    _ => "3".to_string(),
                };
                let current = owned_word.clone();

                if args.diff && prev_annot != "0" {
                    owned_word = prev_annot.clone();
                }
                prev_annot = current.clone();

                annotations.insert(owned_word.clone());
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

    (result, annotations)
}

fn write_rrs(data: &Vec<Vec<String>>, output_path: &str) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;
    writeln!(file, "RR\tannot")?;

    // writing data
    for row in data {
        if row.len() >= 2 {
            writeln!(file, "{}\t{}", row[0], row[1])?;
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    let args = parse_args(&argv);
    println!("args: {:?}", args);
    let mut annotation_store: HashSet<String> = HashSet::new();
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let available_files = fs::read_dir(current_dir).unwrap();
    for (i, entry) in available_files.enumerate() {
        let entry_path = entry.unwrap().path();
        println!("{}, processing {:?}", i, entry_path);
        if entry_path.extension().and_then(|s| s.to_str()) == Some(args.input_extension) {
            let (mut contents, new_annotations) = read_lines(&entry_path.to_str().unwrap(), &args);
            annotation_store.extend(new_annotations);

            // Apply correction if the correct flag is true
            if args.correct {
                correct_processed_rr_intervals(&mut contents);
            }

            let filepath = form_result_path(&entry_path.to_str().unwrap(), &args.output_extension);
            if !args.scout {
                match write_rrs(&contents, &filepath) {
                    Ok(_) => {}
                    Err(e) => eprintln!("encountered error {}  writing {}", e, filepath),
                }
            }
        }
    }
    println!("Unique annotations: {:?}", annotation_store);
    Ok(())
}
