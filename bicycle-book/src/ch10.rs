//! wordcount はシンプルな文字、単語、行の出現頻度の計算機能を提供します。
//! 詳しくは [`count`] 関数のドキュメントを見てください。

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`] で使うオプション
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度を数える
    Char,
    /// 単語ごとに頻度を数える
    Word,
    /// 行ごとに頻度を数える
    Line,
}

/// オプションのデフォルトは [`CountOption::Word`]
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// input から 1 行ずつ UFT-8 文字列を読み込み、頻度を数える
///
/// 頻度を数える対象はオプションによって制御される
/// * [`CountOption::Char`]: Unicode の 1 文字ごと
/// * [`CountOption::Word`]: 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Line`]: \n または \r\n で区切られた 1 行ごと
///
/// # Examples
///
/// 入力中の単語の出現頻度を数える例
///
/// ```
/// use std::io::Cursor;
/// use bicycle_book::ch10::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
/// 入力が UTF-8 でフォーマットされていない場合にパニックする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value: expr),*}) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works() {
        let freqs = count(Cursor::new("aa bb cc bb"), CountOption::Word);

        assert_map!(freqs, {"aa" => 1, "bb" => 2, "cc" => 1});
    }
}
