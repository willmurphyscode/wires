use std::io::{self, Read, BufReader};
use std::fs::File;
use std::process; 

extern crate clap;
#[macro_use]
extern crate nom;
use clap::{Arg, App};
mod wires;
mod z_string;

fn parse_options() -> wires::Options {
    let matches = App::new("Wires: strings in Rust")
                        .version("0.1")
                        .about("Gets ascii strings out of files")
                        .arg(Arg::with_name("FILE")
                            .help("The file to get strings out of")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("radix")
                            .short("t")
                            .long("radix")
                            .help("Print offset where strings occur?")
                            .takes_value(true)
                            .required(false))
                        .arg(Arg::with_name("bytes")
                            .short("n")
                            .long("bytes")
                            .takes_value(true))
                        .get_matches(); 

    let path = matches.value_of("FILE").expect("Please provide a path");
    let option_radix = matches.value_of("radix");
    let radix = wires::string_to_offset_radix(option_radix).expect("Invalid offset radix specified");

    let option_bytes = matches.value_of("bytes");
    let mut bytes = 4usize; 
    
    if let Some(string) = option_bytes {
        bytes = str::parse(string).expect("-n must take an integer value");
    }

    wires::Options { 
        print_offset: radix, 
        match_length: bytes,
        path: path.to_string()
    }
}


fn main() {

    let options = parse_options(); 
    let fs_result = File::open(&options.path);
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


