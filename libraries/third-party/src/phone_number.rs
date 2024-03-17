#[cfg(test)]
mod tests {
    use phonenumber::{country::Id, Mode};
    use rstest::rstest;

    #[rstest]
    #[case("+14134870806")]
    #[case("+1-413-487-0806")]
    #[case("+1 (413) 487-0806")]
    fn test_parse_international_us_phone_number(#[case] phone_number: &str) {
        let phone_number = phonenumber::parse(None, phone_number).unwrap();

        assert_eq!(phone_number.country().id(), Some(Id::US));
        assert_eq!(phone_number.national().to_string(), "4134870806");
        assert_eq!(phone_number.to_string(), "+14134870806");
    }

    #[rstest]
    #[case("+81312345678")]
    #[case("+81-3-1234-5678")]
    #[case("+81 (3) 1234-5678")]
    fn test_parse_international_jp_phone_number(#[case] phone_number: &str) {
        let phone_number = phonenumber::parse(None, phone_number).unwrap();

        assert_eq!(phone_number.country().id(), Some(Id::JP));
        assert_eq!(phone_number.national().to_string(), "312345678");
        assert_eq!(phone_number.to_string(), "+81312345678");
    }

    #[rstest]
    #[case("0312345678")]
    #[case("03-1234-5678")]
    #[case("03(1234)5678")]
    #[case("03 (1234) 5678")]
    #[case("+81312345678")]
    fn test_parse_japanese_phone_number(#[case] phone_number: &str) {
        let phone_number = phonenumber::parse(Some(Id::JP), phone_number).unwrap();

        assert_eq!(phone_number.country().id(), Some(Id::JP));
        assert_eq!(phone_number.national().to_string(), "312345678".to_string());
        assert_eq!(phone_number.to_string(), "+81312345678");
    }

    #[rstest]
    #[case(Mode::E164, "+81312345678")]
    #[case(Mode::International, "+81 3-1234-5678")]
    #[case(Mode::National, "03-1234-5678")]
    #[case(Mode::Rfc3966, "tel:+81-3-1234-5678")]
    fn test_format_japanese_phone_number(#[case] mode: Mode, #[case] expected: &str) {
        let phone_number = phonenumber::parse(Some(Id::JP), "03(1234)5678").unwrap();

        assert_eq!(expected, phone_number.format().mode(mode).to_string());
    }
}
