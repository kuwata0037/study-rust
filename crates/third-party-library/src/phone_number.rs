#[cfg(test)]
mod tests {
    use phonenumber::{country::Id, parse, Mode};
    use rstest::rstest;

    #[rstest]
    #[case("+1 (413) 487-0806")]
    #[case("+14134870806")]
    #[case("+81 (3) 1234-5678")]
    #[case("+81-3-1234-5678")]
    #[case("+81312345678")]
    fn test_successful_parse_phone_number(#[case] phone_number: &str) {
        parse(None, phone_number).unwrap();
    }

    #[rstest]
    #[case("0312345678")]
    #[case("03-1234-5678")]
    #[case("03(1234)5678")]
    #[case("03 (1234) 5678")]
    #[case("+81312345678")]
    fn test_successful_parse_japanese_phone_number(#[case] phone_number: &str) {
        parse(Some(Id::JP), phone_number).unwrap();
    }

    #[rstest]
    #[case(Mode::E164, "+81312345678")]
    #[case(Mode::International, "+81 3-1234-5678")]
    #[case(Mode::National, "03-1234-5678")]
    #[case(Mode::Rfc3966, "tel:+81-3-1234-5678")]
    fn test_japanese_phone_number_format(#[case] mode: Mode, #[case] expected: &str) {
        let phone_number = parse(Some(Id::JP), "03(1234)5678").unwrap();
        assert_eq!(
            expected,
            phone_number.format().mode(mode).to_string().as_str()
        );
    }
}
