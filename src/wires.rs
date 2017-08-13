use std::io::Write;
use std::ascii::AsciiExt;
use std::str;

pub fn bytes_to_strings<W: Write>(bytes: &[u8], w:  &mut W) {
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
                        writeln!(w, "{}", string).expect("Failed to write to supplied stream");
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
                writeln!(w, "{}", string).expect("Failed to write to supplied stream.");
            },
            Err(_) => {}
        }
    }
}


#[test]
fn it_reads_strings_from_files() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let expected = Result::Ok("Hello, world.".to_string());
    assert_eq!(strings_from_file("./src/hello.txt", &mut handle), expected);
}

#[test]
fn it_raises_if_no_file_exists() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let actual = strings_from_file("no such file", &mut handle);
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