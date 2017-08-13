use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::ascii::AsciiExt;
use std::str;
use lazy_static; 

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
        // TODO length check. Current method will panic if len() == 0
        self.print_all(&self.strs);
        self.strs[self.strs.len() - 1].clone()
    }

    pub fn clear(&mut self) {
        self.strs.truncate(0);
    }



    pub fn print(&self) {
        self.print_all(&self.strs);
    }

    pub fn join_all(&self) -> String {
        if self.any() {
            self.strs.join("\n")
        } else {
            "".to_string()
        }
    }

    fn any(&self) -> bool {
        self.strs.len() > 0
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
    lazy_static::initialize(&GLOBAL_BUFFER);
    match fs_result {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents : Vec<u8> = Vec::new();
            let read_result = reader.read_to_end(&mut contents);
            match read_result {
                Ok(_) => {
                    bytes_to_strings(&contents);
                    let guard = GLOBAL_BUFFER.lock().unwrap();

                    Result::Ok(guard.join_all())             
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
    let mut current_bytes : Vec<u8> = Vec::new(); 
    let mut guard = GLOBAL_BUFFER.lock().unwrap();
    guard.clear();
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
                        guard.insert(string.to_string());
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
            guard.insert(string.to_string());
        },
        Err(_) => {}
    }
}
}

fn clear_buffer() {
    let mut guard = GLOBAL_BUFFER.lock().unwrap();
    guard.clear();
}

#[test]
fn clear_buffer_always_works() {
    for i in 0..10 {
        let mut guard = GLOBAL_BUFFER.lock().unwrap();
        guard.insert("cheese".to_string());
        drop(guard);
        clear_buffer();
        let cleared_guard = GLOBAL_BUFFER.lock().unwrap();
        assert_eq!("", cleared_guard.join_all());
    }

}


#[test]
fn it_reads_strings_from_files() {
    clear_buffer();
    let expected = Result::Ok("Hello, world.".to_string());
    assert_eq!(strings_from_file("./src/hello.txt"), expected);
}

#[test]
fn it_raises_if_no_file_exists() {
    clear_buffer();   
    let actual = strings_from_file("no such file");
    let expected = Result::Err("Could not open file");
    assert_eq!(expected, actual)
}

#[test]
fn it_pushes_strings_to_global_buffer() {
    clear_buffer(); 
    bytes_to_strings("hello".as_bytes());
    let expected = "hello";
    let guard = GLOBAL_BUFFER.lock().unwrap();
    let actual = guard.get_last();
    assert_eq!(expected, actual);
}

#[test]
fn it_returns_empty_str_on_bad_bytes() {
    clear_buffer();  
    let no_string_bytes = vec![254u8, 254u8, 254u8, 254u8, 254u8, 254u8];
    bytes_to_strings(&no_string_bytes);
    let expected = "".to_string();
    let guard = GLOBAL_BUFFER.lock().unwrap();
    let actual = guard.join_all();
    assert_eq!(expected, actual);    
}