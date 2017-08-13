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
fn it_writes_to_the_buffer() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(Vec::new()); 
    bytes_to_strings("hello".as_bytes(), &mut cursor);
    let expected = "hello\n";
    let vec = cursor.into_inner(); 
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_omits_intermediate_nonsense() {
    use std::io::Cursor;
    let mut cursor = Cursor::new(Vec::new());

    let mut bytes : Vec<u8> = Vec::new();
    for b in "Hello".as_bytes() {
        bytes.push(*b);
    }
    bytes.push(254u8);
    bytes.push(254u8);
    for b in "World".as_bytes() {
        bytes.push(*b);
    }
    bytes_to_strings(&bytes, &mut cursor);
    let expected = "Hello\nWorld\n";
    let vec = cursor.into_inner();
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}