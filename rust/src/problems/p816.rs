pub struct Solution;

use std::borrow::Cow;

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut result = vec![];
        for c in 2..n - 1 {
            for a in 2..=c {
                if (a > 2 && bytes[1] == b'0') || (a < c && bytes[c - 1] == b'0') {
                    continue;
                }
                let x = if a < c {
                    Cow::Owned(format!("{}.{}", &s[1..a], &s[a..c]))
                } else {
                    Cow::Borrowed(&s[1..c])
                };
                for b in c + 1..=n - 1 {
                    if (b > c + 1 && bytes[c] == b'0') || (b < n - 1 && bytes[n - 2] == b'0') {
                        continue;
                    }
                    let y = if b < n - 1 {
                        Cow::Owned(format!("{}.{}", &s[c..b], &s[b..n - 1]))
                    } else {
                        Cow::Borrowed(&s[c..n - 1])
                    };
                    result.push(format!("({x}, {y})"));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::ambiguous_coordinates("(123)".to_string());
        result.sort_unstable();
        let mut expected = ["(1, 23)", "(12, 3)", "(1.2, 3)", "(1, 2.3)"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::ambiguous_coordinates("(00011)".to_string());
        result.sort_unstable();
        let mut expected = ["(0.001, 1)", "(0, 0.011)"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let mut result = Solution::ambiguous_coordinates("(0123)".to_string());
        result.sort_unstable();
        let mut expected = [
            "(0, 123)",
            "(0, 12.3)",
            "(0, 1.23)",
            "(0.1, 23)",
            "(0.1, 2.3)",
            "(0.12, 3)",
        ]
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
