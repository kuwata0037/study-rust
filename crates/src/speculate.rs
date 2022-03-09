#![allow(unused)]

struct IntCloseRange {
    lower: i32,
    upper: i32,
}

impl IntCloseRange {
    fn new(lower: i32, upper: i32) -> Self {
        Self { lower, upper }
    }

    fn lower(&self) -> i32 {
        self.lower
    }

    fn upper(&self) -> i32 {
        self.upper
    }

    fn notation(&self) -> String {
        format!("[{}, {}]", self.lower, self.upper)
    }

    fn includes(&self, arg: i32) -> bool {
        self.lower <= arg && arg <= self.upper
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use speculate::speculate;

    speculate! {
        describe "IntClosedRangeは整数閉区間を表す" {
            #[fixture(lower=3, upper=7)]
            fn fixture(lower: i32, upper: i32) -> IntCloseRange {
                IntCloseRange::new(lower, upper)
            }

            describe "IntCloseRangeは下端点と上端点を持つ" {
                #[rstest]
                fn lowerメソッドは整数閉区間の下端点を返す(fixture:IntCloseRange) {
                    assert_eq!(3, fixture.lower());
                }

                #[rstest]
                fn upperメソッドは整数閉区間の上端点を返す(fixture: IntCloseRange) {
                    assert_eq!(7, fixture.upper());
                }
            }
            describe "IntCloseRangeは整数閉区間の文字列表記を返す" {
                #[rstest(range, expected,
                    case(fixture(3, 7), "[3, 7]"),
                    case(fixture(-2, 3), "[-2, 3]")
                )]
                fn notationメソッドは整数閉区間の文字列表記を返す(range: IntCloseRange, expected: String) {
                    assert_eq!(expected, range.notation());
                }
            }
            describe "IntCloseRangeは指定した整数を含むか判断できる" {
                #[rstest(arg, expected,
                    case(1, false),
                    case(3, true),
                    case(5, true),
                    case(7, true),
                    case(9, false)
                )]
                fn includesメソッドは指定した値が区間内に含まれるか判定しboolを返す(fixture: IntCloseRange, arg: i32, expected: bool) {
                    assert_eq!(expected, fixture.includes(arg));
                }

            }
        }
    }
}
