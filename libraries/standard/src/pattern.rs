#[cfg(test)]
mod tests {
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    #[test]
    fn test_match_literal() {
        let x = 3;
        let result = match x {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "anything",
        };
        assert_eq!(result, "three");
    }

    #[test]
    fn test_match_multi() {
        let x = 1;
        let result = match x {
            1 | 2 => "one or two",
            3 => "three",
            _ => "anything",
        };
        assert_eq!(result, "one or two")
    }

    #[test]
    fn test_match_range_number() {
        let x = 5;
        let result = match x {
            0..=5 => "one through five",
            _ => "something else",
        };
        assert_eq!(result, "one through five");
    }

    #[test]
    fn test_match_range_char() {
        let c = 'j';
        let result = match c {
            'a'..='j' => "early ASCII letter",
            'k'..='z' => "late ASCII letter",
            _ => "something else",
        };
        assert_eq!(result, "early ASCII letter");
    }

    #[test]
    fn test_match_destructuring_struct() {
        let p = Point::new(0, 7);
        let result = match p {
            Point { x, y: 0 } => format!("On the x axis at {x}"),
            Point { x: 0, y } => format!("On the y axis at {y}"),
            Point { x, y } => format!("On neither axis: ({x}, {y})"),
        };
        assert_eq!(result, "On the y axis at 7");
    }

    #[test]
    fn test_match_guard() {
        let inner_value = 5;
        let option = Some(inner_value);
        let result = match option {
            Some(50) => "Got 50".to_string(),
            Some(n) if n == inner_value => format!("Matched, n = {inner_value}"),
            _ => format!("Default case, x = {option:?}"),
        };
        assert_eq!(result, "Matched, n = 5");
    }

    #[test]
    fn test_match_guard_struct() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };
        let result = match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => format!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => "Found an id in another range".to_string(),
            Message::Hello { id } => format!("Found some other id: {id}"),
        };
        assert_eq!(result, "Found an id in range: 5");
    }

    #[test]
    fn test_destructuring_reference() {
        let v = [Point::new(0, 0), Point::new(1, 5), Point::new(10, -3)];
        let sum = v.iter().map(|&Point { x, y }| x * x + y * y).sum::<i32>();
        assert_eq!(sum, 135);
    }

    #[test]
    fn test_ignoring_value() {
        let numbers = (2, 4, 8, 16, 32);
        let (first, .., last) = numbers;
        assert_eq!(first, 2);
        assert_eq!(last, 32);
    }
}
