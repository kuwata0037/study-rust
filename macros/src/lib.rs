#[macro_export]
macro_rules! my_assert_eq {
    ( $left:expr , $right:expr ) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion fialed: `(left == right)` (left: `{:?}`, right: `{:?}`)",
                        left_val, right_val
                    );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! my_vec {
    ( $elem:expr ; $n:expr ) => {
        std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {
        <[_]>::into_vec(Box::new([ $( $x ),* ]))
    };
    ( $( $x:expr ),+ , ) => {
        my_vec![ $( $x ),* ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_assert_eq() {
        my_assert_eq!(1 << 10, 1024);
    }

    #[test]
    fn test_my_vec() {
        my_assert_eq!(my_vec!(1, 10), vec![1, 10]);
        my_assert_eq!(my_vec!(1, 2, 3), vec![1, 2, 3]);
        my_assert_eq!(my_vec!(1, 2, 3,), vec![1, 2, 3,]);
    }
}
