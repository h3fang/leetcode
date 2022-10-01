pub struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let num = number
            .as_bytes()
            .iter()
            .filter(|&&b| b.is_ascii_digit())
            .cloned()
            .collect::<Vec<_>>();
        let n = num.len();
        match n % 3 {
            1 => {
                let mut r =
                    num.chunks_exact(3)
                        .fold(String::with_capacity(num.len() * 2), |mut acc, w| {
                            acc.push_str(unsafe { std::str::from_utf8_unchecked(w) });
                            acc.push('-');
                            acc
                        });
                r.pop();
                let a = r.pop().unwrap();
                r.push('-');
                r.push(a);
                r.push(*num.last().unwrap() as char);
                r
            }
            _ => {
                let mut r =
                    num.chunks(3)
                        .fold(String::with_capacity(num.len() * 2), |mut acc, w| {
                            acc.push_str(unsafe { std::str::from_utf8_unchecked(w) });
                            acc.push('-');
                            acc
                        });
                r.pop();
                r
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "123-456",
            Solution::reformat_number("1-23-45 6".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "123-45-67",
            Solution::reformat_number("123 4-567".to_string())
        );
    }
}
