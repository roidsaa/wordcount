//! wordcount はシンプルな文字、単語、行の出現頻度の計数機能を提供する。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見て下さい。

use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度を数える
    Char,
    /// 単語ごとに頻度を数える
    Word,
    /// 行ごとに頻度を数える
    Line,
}

/// オプションのデフォルトは [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// input から1行ずつ UTF-8 文字列を読み込み、頻度を数える
///
/// 頻度を数える対象はオプションによって制御される
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicode の1文字ごと
/// * [`CountOption::Char`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Char`](enum.CountOption.html#variant.Line): \n または \r\n で区切られた1行ごと
///
///
/// # Examples
/// 入力中の単語の出現頻度を数える例
///
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
///
/// ```
///
/// # Panics
///
/// 入力がUTF-8でフォーマットされていない場合にパニックする
///
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            },
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            },
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value:expr), *}) => {
            $(assert_eq!($expr[$key], $value));*
        }
    }


    #[test]
    fn word_count_works() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);

        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }

    #[test]
    fn word_count_works2() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("cc".to_string(), 1);
        exp.insert("dd".to_string(), 1);

        assert_eq!(count(Cursor::new("aa  cc dd"), CountOption::Word), exp);
    }

    #[test]
    fn word_count_works3() {
        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);
        assert_eq!(freqs.len(), 3);
        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1});
    }

    #[test]
    #[should_panic]
    fn word_count_do_not_contain_unkonwn_words() {
        count(
            Cursor::new([
                b'a', // a
                0xf0, 0x90, 0x80, // でたらめなバイト列
                0xe3, 0x81, 0x82, // あ
            ]),
            CountOption::Word,
        );
    }
}

