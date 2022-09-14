use std::{env, fs, path};
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("WIP");
        return;
    }

    let option_arg = &args[1];

    if option_arg == "-v" || option_arg == "--version" {
        println!("rost_gen version 0.1");
    } else if option_arg == "-h" || option_arg == "--help" {
        println!("Usage: rost_gen [OPTIONS]");
        println!("Options:");
        println!("\t-v, --version\t\t\tPrint the tool name and version");
        println!("\t-h, --help\t\t\tPrint help message");
    } else if option_arg == "-i" || option_arg == "--input" {
        let input_path = &args[2];
        let path =  path::Path::new(input_path);

        if !path.exists() {
            println!("Invalid path: No file or directory found at {input_path}");
            return;
        }

        if path::Path::new("./dist").exists() {
            fs::remove_dir_all("./dist").expect("Delete existing output directory")
        }
        fs::create_dir_all("./dist").expect("Create output directory");
        
        if path.is_dir() {
            let dir = fs::read_dir(input_path);
            println!("WIP");
            return;
        }

        if path.is_file() {
            if path.extension().unwrap().to_str().unwrap() != "txt" { 
                println!("Only .txt files are accepted");
            }
    
            let in_file = fs::File::open(input_path).expect(&format!("Open file at {input_path}"));
            let mut buf_reader = io::BufReader::new(in_file);
            let html_file_name = path.file_stem().unwrap().to_str().unwrap();
            let mut read_buffer = String::new();
            let mut read_bytes: u64 = 0;
    
            while read_bytes < fs::metadata(input_path).expect("Read input file").len() {
                read_bytes += buf_reader.read_line(&mut read_buffer).expect("Read input file") as u64;
                fs::write(format!("./dist/{html_file_name}.html"), read_buffer.clone()).expect("Generate html file");
            }
        }
        
    }
}
