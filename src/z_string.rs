use nom::IResult;


named!( take_5_bits<u8>, bits!( take_bits!( u8, 5 ) ) );

fn char_from_5_bits(fiver: u8) -> char {
    let alphabet_table: Vec<char> = vec![
        ' ', '?', '?', '?', '?', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z' ];
    alphabet_table[fiver as usize]
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