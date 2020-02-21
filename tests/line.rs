use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn line_count_works() {
    let input = Cursor::new(
        r#"Tokyo, Japan
Kyoto, Japan
Tokyo, Japan
Shanghai, China
"#
    );

    let freq = count(input, CountOption::Line);

    assert_map!(freq, {
        "Tokyo, Japan" => 2,
        "Kyoto, Japan" => 1,
        "Shanghai, China" => 1
    });
}
