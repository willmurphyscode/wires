use std::io::{stderr, Read, Write, ErrorKind};
use std::ascii::AsciiExt;
use std::str;
use std::fmt;
use std::process;

pub struct Options {
   pub print_offset: OffsetRadix,
   pub match_length: usize,
   pub path: String
}

#[derive(Clone, Copy)]
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
            "o" => Ok(OffsetRadix::Octal),
            "d" => Ok(OffsetRadix::Decimal),
            _ => Err(())
        },
        None => Ok(OffsetRadix::None)
    }

}

pub fn bytes_to_strings<R: Read, W: Write>(stream: &mut R, w:  &mut W, opts: &Options) {
    let min_consecutive_chars = opts.match_length;
    let mut current_bytes : Vec<u8> = Vec::new(); 

    let mut offset = 0usize; 
    for byte_result in stream.bytes() {
        match byte_result {
            Ok(b) => {
                if b.is_ascii() {
                    current_bytes.push(b);
                }
                else  {
                    if current_bytes.len() >= min_consecutive_chars {
                        let result = str::from_utf8(&current_bytes);
                        match result {
                            Ok(string) => {
                                let offset_string = offset_string(offset, opts.print_offset);
                                let write_result = writeln!(w, "{}{}", offset_string, string);
                                match write_result {
                                    Ok(_) => (),
                                    Err(e) => match e.kind() {
                                        ErrorKind::BrokenPipe => break,
                                        _ => {
                                            writeln!(stderr(), "{}", e).unwrap();
                                            process::exit(1);
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                    current_bytes.truncate(0);
                    offset = offset + 1;
                }

            }
            Err(e) => {
                writeln!(stderr(), "{}", e).unwrap();
                process::exit(1);
            }
        }
    }
    if current_bytes.len() >= min_consecutive_chars {
        let result = str::from_utf8(&current_bytes);
        match result {
            Ok(string) => {
                let offset_string = offset_string(offset, opts.print_offset);
                let write_result = writeln!(w, "{}{}", offset_string, string);
                match write_result {
                    Ok(_) => (),
                    Err(e) => match e.kind() {
                        ErrorKind::BrokenPipe => (),
                        _ => {
                            writeln!(stderr(), "{}", e).unwrap();                            
                            process::exit(1);
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }
}

fn offset_string(offset: usize, radix: OffsetRadix) -> String {
    let mut output = "".to_string();
    match radix {
        OffsetRadix::Hex => fmt::write(&mut output, format_args!("0x{:X}: ", offset)).unwrap(),
        OffsetRadix::Octal => fmt::write(&mut output, format_args!("0o{:o}: ", offset)).unwrap(),
        OffsetRadix::Decimal => fmt::write(&mut output, format_args!("{}: ", offset)).unwrap(),
        OffsetRadix::None => ()
    }

    output
}




#[test]
fn it_writes_to_the_buffer() {

    use std::io::Cursor;

    let opts = Options {
        print_offset: OffsetRadix::None,
        match_length: 3,
        path: "".to_string()
    };

    let mut cursor = Cursor::new(Vec::new());
    let  input_vec = "hello".as_bytes();
    let mut slice : &[u8] = input_vec;
    bytes_to_strings(&mut slice, &mut cursor, &opts);
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
        print_offset: OffsetRadix::None,
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
    let mut slice : &[u8] = bytes.as_mut(); 

    bytes_to_strings(&mut slice, &mut cursor, &opts);
    let expected = "Hello\nWorld\n";
    let vec = cursor.into_inner();
    let actual = String::from_utf8(vec).unwrap();
    assert_eq!(expected, actual);
}
