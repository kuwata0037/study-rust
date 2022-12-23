use bicycle_book::ch10::{count, CountOption};
use std::io::Cursor;

macro_rules! assert_map {
    ($expr: expr, {$($key: expr => $value: expr),*}) => {
        $(assert_eq!($expr[$key], $value));*
    };
}

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");
    let freq = count(input, CountOption::Char);
    assert_map!(freq,
        {
            "a" => 6,
            "b" => 2,
            "c" => 1,
            "d" => 2,
            "r" => 2
        }
    );
}
