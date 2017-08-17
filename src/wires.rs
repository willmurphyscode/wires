use std::io::Write;
use std::ascii::AsciiExt;
use std::str;
use std::fmt;

#[derive(Debug)]
pub struct Options {
   pub print_offset: bool,
   pub match_length: usize,
   pub path: String
}

pub enum OffsetRadix {
    None,
    Hex,
    Octal,
    Decimal
}

pub fn string_to_offset_radix(input: Option<&str>) -> Result<OffsetRadix, ()> {
    match input{
        Some(string) => match string {
            "x" => Ok(OffsetRadix::Hex),
            "o" => Ok(OffsetRadix::Hex),
            "d" => Ok(OffsetRadix::Decimal),
            _ => Err(())
        },
        None => Ok(OffsetRadix::None)
    }

}

pub fn bytes_to_strings<W: Write>(bytes: &[u8], w:  &mut W, opts: &Options) {
    let min_consecutive_chars = opts.match_length;
    let mut current_bytes : Vec<u8> = Vec::new(); 

    let mut offset = 0usize; 

    for b in bytes {
        if b.is_ascii() {
            current_bytes.push(*b);
        }
        else  {
            if current_bytes.len() >= min_consecutive_chars {
                let result = str::from_utf8(&current_bytes);
                match result {
                    Ok(string) => {
                        let offset_string = offset_string(offset, opts.print_offset);
                        writeln!(w, "{}{}", offset_string, string).expect("Failed to write to supplied stream");
                    },
                    Err(_) => {}
                }
            }
            current_bytes.truncate(0);
            offset = offset + 1; 
        }
    }
    if current_bytes.len() >= min_consecutive_chars {
        let result = str::from_utf8(&current_bytes);
        match result {
            Ok(string) => {
                let offset_string = offset_string(offset, opts.print_offset);
                writeln!(w, "{}{}", offset_string, string).expect("Failed to write to supplied stream");
            },
            Err(_) => {}
        }
    }
}

fn offset_string(offset: usize, print_offset: bool) -> String {
    let mut output = "".to_string();   
    if print_offset {
        fmt::write(&mut output, format_args!("0x{:X}: ", offset)).unwrap();
    }
    output
}




#[test]
fn it_writes_to_the_buffer() {

    use std::io::Cursor;

    let opts = Options {
        print_offset: false,
        match_length: 3,
        path: "".to_string()
    };

    let mut cursor = Cursor::new(Vec::new()); 
    bytes_to_strings("hello".as_bytes(), &mut cursor, &opts);
    let expected = "hello\n";
    let vec = cursor.into_inner(); 
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn it_omits_intermediate_nonsense() {
    use std::io::Cursor;
    let mut cursor = Cursor::new(Vec::new());

    let opts = Options {
        print_offset: false,
        match_length: 3,
        path: "".to_string()
    };

    let mut bytes : Vec<u8> = Vec::new();
    for b in "Hello".as_bytes() {
        bytes.push(*b);
    }
    bytes.push(254u8);
    bytes.push(254u8);
    for b in "World".as_bytes() {
        bytes.push(*b);
    }
    bytes_to_strings(&bytes, &mut cursor, &opts);
    let expected = "Hello\nWorld\n";
    let vec = cursor.into_inner();
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}