use std::env;

mod wires;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let result = wires::strings_from_file(&args[1]);
        match result {
            Ok(string) => println!("{}", string),
            Err(msg) => println!("An error occurred: {}", msg)
        }
    } else {
        println!("Please specify a path");
    }
}

