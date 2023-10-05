use std::{env, fs, io};
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{Duration, Instant};
use zip::read::ZipFile;

fn main() {
    let start = Instant::now();
    real_main();
    println!("Took {:?}!", start.elapsed())
}

fn real_main() {
    let args: Vec<String> = env::args().collect();

    let program_name = args.first().expect("Program path isn't present!")
        .split('\\').last().expect("Failed to get program name!");

    if args.len() == 1 {
        println!("ERROR! USAGE: {} [input] [output_dir](optional)", program_name);
        exit(0)
    }

    let input_path = PathBuf::from(&args[1]);

    let out_path = if args.len() < 3 {
        None
    } else {
        Some(args[2].to_string())
    };

    let input_reader = BufReader::new(
        fs::File::open(input_path).expect("File doesn't exist!")
    );

    let mut zip_archive = zip::ZipArchive::new(input_reader)
        .expect("Failed to create zip archive!");

    for i in 0..zip_archive.len() {
        let mut zip_entry: ZipFile = zip_archive.by_index(i).expect("Failed to get file from zip!");

        let entry_path: String = zip_entry.enclosed_name().expect("Failed to get entry path!").to_str().unwrap().to_string();

        let mut path_segments: Vec<String> = entry_path.split('/').map(|str| str.to_string()).collect();

        if let Some(ref out_path) = out_path {
            path_segments[0] = out_path.clone();
        }

        let path_string = path_segments.join("/");

        if path_string.ends_with('/') {
            fs::create_dir_all(path_string).expect("Failed to create directories!");
        } else {
            let path = Path::new(&path_string);
            let parent_path = path.parent().expect("Failed to get parent!");

            fs::create_dir_all(parent_path).expect("Failed to create parent directories!");

            let mut file = fs::File::create(path).expect("Failed to create file!");
            io::copy(&mut zip_entry, &mut file).expect("Failed to copy file content!");
        }
    }
}
