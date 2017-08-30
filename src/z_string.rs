use nom::IResult;

#[derive(Debug)]
struct ZWord {
    first: u8,
    second: u8,
    third: u8,
    last_bit: u8
}


named!( take_5_bits<u8>, bits!( take_bits!( u8, 5 ) ) );

named!( take_z_word<&[u8],ZWord>,
    bits!(
        do_parse!(
            first: take_bits!( u8, 5 ) >>
            second: take_bits!( u8, 5 ) >>
            third: take_bits!( u8, 5 ) >>
            last_bit: take_bits!( u8, 1 ) >>
            (
                ZWord {
                    first: first,
                    second: second,
                    third: third,
                    last_bit: last_bit
                }
            )
        )
    )   
);

fn char_from_5_bits(fiver: u8) -> char {
    let alphabet_table: Vec<char> = vec![
        ' ', '?', '?', '?', '?', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z' ];
    alphabet_table[fiver as usize]
}

/* The other two alphabet tables
[| " "; "?"; "?"; "?"; "?"; "?"; "A"; "B"; "C"; "D"; "E"; "F"; "G"; "H"; "I"; "J";
   "K"; "L"; "M"; "N"; "O"; "P"; "Q"; "R"; "S"; "T"; "U"; "V"; "W"; "X"; "Y"; "Z" |];
[| " "; "?"; "?"; "?"; "?"; "?"; "?"; "\n"; "0"; "1"; "2"; "3"; "4"; "5"; "6"; "7";
   "8"; "9"; "."; ","; "!"; "?"; "_"; "#"; "'"; "\""; "/"; "\\"; "-"; ":"; "("; ")" |] |]
*/

fn z_string_fragment(bytes: &[u8]) -> String {
    if let IResult::Done(_, word) = take_z_word(bytes) {
        let ZWord { first: a, second: b, third: c, last_bit: _ } = word; 
        let chars = vec![
            char_from_5_bits(a),
            char_from_5_bits(b),
            char_from_5_bits(c)
        ];
        let s : String = chars.into_iter().collect();
        return s;
    }
    //TODO handle error case
    "NOT IMPLEMENTED".to_string()
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