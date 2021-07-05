pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid syntax"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("5", 5)]
    #[case("50", 50)]
    #[case("-50", -50)]
    fn test_one_operand(#[case] input: &str, #[case] expected: i32) {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval(input), expected);
    }

    #[rstest]
    #[case("2 3 +", 5)]
    #[case("2 3 *", 6)]
    #[case("2 3 -", -1)]
    #[case("2 3 /", 0)]
    #[case("2 3 %", 2)]
    fn test_two_operand(#[case] input: &str, #[case] expected: i32) {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval(input), expected);
    }

    #[rstest]
    #[should_panic]
    fn test_panic() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
    }
}
