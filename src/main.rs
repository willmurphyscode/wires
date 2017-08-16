use std::io::{self, Read, BufReader};
use std::fs::File;
use std::process; 

extern crate clap;
use clap::{Arg, App};
mod wires;




fn main() {
    let matches = App::new("Wires: strings in Rust")
                        .version("0.1")
                        .about("Gets ascii strings out of files")
                        .arg(Arg::with_name("FILE")
                            .help("The file to get strings out of")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("offset")
                            .short("t")
                            .long("radix")
                            .help("Print offset where strings occur?")
                            .required(false))
                        .arg(Arg::with_name("bytes")
                            .short("n")
                            .long("bytes")
                            .takes_value(true))
                        .get_matches(); 

    let path = matches.value_of("FILE").expect("Please provide a path");
    let print_offset = matches.is_present("offset");

    let option_bytes = matches.value_of("bytes");
    let mut bytes = 3usize; 
    match option_bytes {
        Some(string) => {
            bytes = str::parse(string).expect("-n must take an integer value");
        },
        None => {
            println!("no bytes")
        }
    }

    let options = wires::Options { 
        print_offset: print_offset, 
        match_length: bytes 
    };

    let fs_result = File::open(&path);
    match fs_result {
        Ok(file) =>  {
            let mut reader = BufReader::new(file);
            let mut contents : Vec<u8> = Vec::new();
            let read_result = reader.read_to_end(&mut contents);
            match read_result {
                Ok(_) => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    wires::bytes_to_strings(&contents, &mut handle, &options);
                },
                Err(_) => {
                    println!("An error occurred reading the buffer");
                    process::exit(1);
                }
            }
        },
        Err(_) => { 
            println!("Could not read from file.");
            process::exit(1);
        }
    }
}


