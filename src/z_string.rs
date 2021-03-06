#![allow(dead_code)]

use nom::IResult;
use std::io::{stderr, Write, ErrorKind};
use std::process;
use std::fmt;
use wires::{OffsetRadix, Options, StringExtractor};

#[derive(Debug)]
struct ZWord {
    first: u8,
    second: u8,
    third: u8,
    word_end_bit: u8
}


impl ZWord {
    pub fn to_string_with_last_byte(&self, previous_byte: u8) -> (String, u8) {
        let ZWord { first: a, second: b, third: c, .. } = *self;
        let chars = vec![
            char_from_5_bits(a, previous_byte),
            char_from_5_bits(b, a),
            char_from_5_bits(c, b)
        ];
        let s : String = chars.into_iter().collect();   
        (s, c)
    }
}

pub struct ZorkStringExtractor {

}

impl ZorkStringExtractor {
    fn read_until_break(&self, bytes: &[u8], collection: &mut Vec<ZWord>) -> usize {
        let mut bytes_consumed = 0usize; 
        let mut slice = bytes; 
        loop {
            let result = take_z_word(slice);
            if let IResult::Done(rest, word) = result {
                bytes_consumed += 2; 
                let should_break = word.word_end_bit == 1 || rest.len() <= 2; 
                collection.push(word);
                slice = rest;
                if should_break { break; }            
            }
        }
        bytes_consumed
    }

    fn dump_zword_vec(&self, words: &[ZWord]) -> String{
        let mut previous_byte = 0u8;
        let mut strings: Vec<String> = Vec::new();
        for word in words.iter() {
            let (string, last_byte) = word.to_string_with_last_byte(previous_byte);
            previous_byte = last_byte;
            strings.push(string);
        }

        strings.concat()
    }
}

impl StringExtractor for ZorkStringExtractor {
    fn bytes_to_strings<W: Write>(&self, bytes: &[u8], writer: &mut W, options: &Options) {
        let length = bytes.len();
        let mut offset = 0usize; 
        let mut collection : Vec<ZWord> = Vec::new();

        while offset < (length - 2) {
            let bytes_consumed = self.read_until_break(&bytes[offset..], &mut collection);
            let o_string = self.offset_string(offset, options.print_offset);
            let string = self.dump_zword_vec(&collection); 
            let write_result = writeln!(writer, "{}{}", o_string, string);
            if let Err(e) = write_result {
                match e.kind() {
                    ErrorKind::BrokenPipe => break,
                    _ => {
                        writeln!(stderr(), "{}", e).unwrap();
                        process::exit(1);
                    }
                }
            }
            offset += bytes_consumed; 
            collection.truncate(0);
        }
    }

    fn offset_string(&self, offset: usize, radix: OffsetRadix) -> String {
        let mut output = "".to_string();
        match radix {
            OffsetRadix::Hex => fmt::write(&mut output, format_args!("0x{:X}: ", offset)).unwrap(),
            OffsetRadix::Octal => fmt::write(&mut output, format_args!("0o{:o}: ", offset)).unwrap(),
            OffsetRadix::Decimal => fmt::write(&mut output, format_args!("{}: ", offset)).unwrap(),
            OffsetRadix::None => ()
        }

        output
    }
}


named!( take_5_bits<u8>, bits!( take_bits!( u8, 5 ) ) );

named!( take_z_word<&[u8],ZWord>,
    bits!(
        do_parse!(
            word_end_bit: take_bits!( u8, 1 ) >>            
            first: take_bits!( u8, 5 ) >>
            second: take_bits!( u8, 5 ) >>
            third: take_bits!( u8, 5 ) >>
            (
                ZWord {
                    first: first,
                    second: second,
                    third: third,
                    word_end_bit: word_end_bit
                }
            )
        )
    )   
);

fn char_from_5_bits(fiver: u8, previous_byte: u8) -> char {

    let alphabet_table: Vec<char> = vec![
        ' ', '?', '?', '?', '?', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z' ];
    let uppercase_chars: Vec<char> = vec![
        ' ', '?', '?', '?', '?', '?', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z' ];

    let punctuation_chars: Vec<char> = vec![
        ' ', '?', '?', '?', '?', '?', '?', '\n', '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', '.', ',', '!', '?', '_', '#', '\'', '"', '/', '\\', '-', ':', '(', ')' ];

    match previous_byte {
        4u8 => uppercase_chars[fiver as usize],
        5u8 => punctuation_chars[fiver as usize],
        _ => alphabet_table[fiver as usize]
    }
}

fn read_until_break(bytes: &[u8], collection: &mut Vec<ZWord>) {
    let mut slice = bytes; 
    loop {
        let result = take_z_word(slice);
        if let IResult::Done(rest, word) = result {
            let should_break = word.word_end_bit == 1 || rest.len() <= 2; 
            collection.push(word);
            slice = rest;
            if should_break { break; }            
        }
    }
}

#[test]
fn it_takes_bits() {
    let input = vec![0b10101010, 0b11110000, 0b00110011];
    let sl    = &input[..];
    assert_eq!(take_5_bits( sl ), IResult::Done(&sl[1..], 21) );
}

#[test]
fn it_can_get_letter() {
    let starts_with_six = vec! [0b00110111, 0b11110000, 0b11110001];
    let sl = &starts_with_six[..];
    let expected = 'a';
    let actual = char_from_5_bits( take_5_bits(sl).to_result().unwrap() );
    assert_eq!(expected, actual);
}

#[test]
fn it_can_parse_the() {
    let bytes_for_the = vec! [0b1100_1011u8, 0b0101_0101u8];
    let sl = &bytes_for_the[..];
    let expected = "the".to_string();
    let actual = z_string_fragment(sl);
    assert_eq!(expected, actual);
}

#[test]
fn read_until_break_breaks() {
    let bytes_for_the = vec! [0b1100_1011u8, 0b0101_0100u8, 0b1100_1011u8, 0b0101_0101u8, 0b1100_1011u8, 0b0101_0101u8];
    let sl = &bytes_for_the[..];
    let expected = "thethe".to_string();
    let mut collection : Vec<ZWord> = Vec::new();
    read_until_break(sl, &mut collection);
    println!("{:?}", collection);
    let strings : Vec<String> = collection
        .into_iter()
        .map(|zword| zword.to_string())
        .collect();

    let actual = strings.concat();

    assert_eq!(expected, actual); 
}