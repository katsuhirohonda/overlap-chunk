use overlap_chunk::{ChunkOptions, chunk_text};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    // Parse command line options
    let mut chunk_size = 100; // Default chunk size
    let mut overlap_percentage = 0; // Default is no overlap
    let mut input_file = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-s" | "--size" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(size) => chunk_size = size,
                        Err(_) => {
                            eprintln!("Error: Chunk size must be a number");
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --size option requires a value");
                    process::exit(1);
                }
            }
            "-o" | "--overlap" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u8>() {
                        Ok(percentage) if percentage <= 90 => overlap_percentage = percentage,
                        Ok(_) => {
                            eprintln!("Error: Overlap percentage must be between 0 and 90");
                            process::exit(1);
                        }
                        Err(_) => {
                            eprintln!("Error: Overlap must be a number between 0 and 90");
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --overlap option requires a value");
                    process::exit(1);
                }
            }
            arg if !arg.starts_with('-') => {
                input_file = Some(arg.to_string());
                i += 1;
            }
            _ => {
                eprintln!("Error: Unknown option: {}", args[i]);
                print_usage();
                process::exit(1);
            }
        }
    }

    // Read text
    let text = match input_file {
        Some(filename) => match fs::read_to_string(&filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                process::exit(1);
            }
        },
        None => {
            // Read from standard input
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer,
                Err(e) => {
                    eprintln!("Error reading from standard input: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    // Set options
    let options = ChunkOptions { overlap_percentage };

    // Split text into chunks
    let chunks = chunk_text(&text, chunk_size, Some(options));

    // Output results
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {}", i + 1, chunk);
    }
}

fn print_usage() {
    println!("Usage: overlap-chunk [OPTIONS] [FILE]");
    println!("  If no file is specified, read from standard input");
    println!();
    println!("Options:");
    println!("  -h, --help              Display this help message");
    println!("  -s, --size SIZE         Specify chunk size (default: 100)");
    println!("  -o, --overlap PERCENT   Specify overlap percentage between 0 and 90 (default: 0)");
}
