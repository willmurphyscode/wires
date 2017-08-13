use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
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

    pub fn get_last(&self) -> String {
        self.print_all(&self.strs);
        self.strs[self.strs.len() - 1].clone()
    }

    fn print_all(&self, strs : &Vec<String>) {
        let mut ix = 0; 
        for s in strs {
            println!("{}: {}", ix, s);
            ix = ix + 1; 
        }
    }
}

pub fn strings_from_file<'a>(path: &str) -> Result<String, &'a str> {
    let fs_result = File::open(path);
    match fs_result {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents : Vec<u8> = Vec::new();
            let read_result = reader.read_to_end(&mut contents);
            match read_result {
                Ok(_) => {
                    bytes_to_strings(&contents);
                    let guard = GLOBAL_BUFFER.lock().unwrap();
                    Result::Ok(guard.get_last())             
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

fn bytes_to_strings(bytes: &[u8]) {
    let min_consecutive_chars = 3;
    let mut guard = GLOBAL_BUFFER.lock().unwrap();
    if bytes.is_ascii() {
        let result = str::from_utf8(&bytes);
        match result {
            Ok(string) => {
                println!("Found string: {}", string.clone());
                guard.insert(string.to_string());
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
fn it_pushes_strings_to_global_buffer() {
    bytes_to_strings("hi".as_bytes());
    let expected = "hi";
    let guard = GLOBAL_BUFFER.lock().unwrap();
    let actual = guard.get_last();
    assert_eq!(expected, actual);
}