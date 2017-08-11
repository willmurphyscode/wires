use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn strings_from_file<'a>(path: &str) -> Result<String, &'a str> {
    let fs_result = File::open(path);
    match fs_result {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let read_result = reader.read_to_string(&mut contents);
            match read_result {
                Ok(_) => {
                    Result::Ok(contents.to_string())              
                },
                Err(error) => {
                    print!("{:?}", error);                    
                    Result::Err("Could not read file")
                }
            }
        },
        Err(error) => {
            print!("{:?}", error);
            Result::Err("Could not open file")
        }
    }
}


#[test]
fn it_reads_strings_from_files() {
    let expected = Result::Ok("Hello, world.".to_string());
    assert_eq!(strings_from_file("./src/hello.txt"), expected);
}

#[test]
fn it_raises_if_no_file_exists() {
    let actual = strings_from_file("no such file");
    let expected = Result::Err("Could not open file");
    assert_eq!(expected, actual)
}