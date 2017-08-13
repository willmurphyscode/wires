use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::Cursor;
use std::ascii::AsciiExt;
use std::str;

use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_BUFFER: Mutex<Buffer> = Mutex::new(Buffer::new());
}

pub struct Buffer {
    strs: Vec<String>
}

impl Buffer {
    pub fn new() -> Buffer { Buffer { strs: Vec::new() } }

    pub fn insert(&mut self, s: String) {
        self.strs.push(s)
    }
    pub fn get_by_index(&self, ix: usize) -> String {
        self.strs[ix].clone()
    }
}

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

fn bytes_to_strings<W: Write>(bytes: &[u8], w:  &mut W) {
    let min_consecutive_chars = 3;
    let mut current_bytes : Vec<u8> = Vec::new(); 

    for b in bytes {
        if b.is_ascii() {
            current_bytes.push(*b);
        }
        else  {
            if current_bytes.len() >= min_consecutive_chars {
                let result = str::from_utf8(&current_bytes);
                match result {
                    Ok(string) => {
                        println!("Found string: {}", string.clone());
                        writeln!(w, "{}", string);
                    },
                    Err(_) => {}
                }
            }
            current_bytes.truncate(0)
        }
    }
    if current_bytes.len() >= min_consecutive_chars {
        let result = str::from_utf8(&current_bytes);
        match result {
            Ok(string) => {
                println!("Found string: {}", string.clone());
                writeln!(w, "{}", string);
            },
            Err(_) => {}
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

#[test]
fn it_writes_to_the_buffer() {
    let mut cursor = Cursor::new(Vec::new()); 
    bytes_to_strings("hello".as_bytes(), &mut cursor);
    let expected = "hello\n";
    let vec = cursor.into_inner(); 
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}