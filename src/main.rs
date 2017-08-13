use std::env;
use std::io::{self, Read, BufReader};
use std::fs::File;

mod wires;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // let result = wires::strings_from_file(&args[1]);
        let fs_result = File::open(&args[1]);
        match fs_result {
            Ok(file) =>  {
                let mut reader = BufReader::new(file);
                let mut contents : Vec<u8> = Vec::new();
                let read_result = reader.read_to_end(&mut contents);
                match read_result {
                    Ok(_) => {
                        let stdout = io::stdout();
                        let mut handle = stdout.lock();
                        wires::bytes_to_strings(&contents, &mut handle);
                    },
                    Err(_) => println!("An error occurred reading the buffer")
                }
            },
            Err(_) => println!("Could not read from file.")

        }

    } else {
        println!("Please specify a path");
    }
}

