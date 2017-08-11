pub fn strings_from_file<'a>(path: &str) -> Result<&'a str, &'a str> {
    
    Result::Ok("Hello, world")
}

#[test]
fn it_reads_strings_from_files() {
    let expected = Result::Ok("Hello, world");
    assert_eq!(strings_from_file("hello.txt"), expected);
}

#[test]
fn it_raises_if_no_file_exists() {
    let actual = strings_from_file("no such file");
    let expected = Result::Err("Could not read file at specified path");
    assert_eq!(expected, actual)
}