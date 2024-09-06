pub struct Solution;

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x /= radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        for b in 2..=n - 2 {
            let s = format_radix(n as u32, b as u32);
            let s = s.as_bytes();
            let len = s.len();
            for i in 0..len / 2 {
                if s[i] != s[len - 1 - i] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::is_strictly_palindromic(9));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_strictly_palindromic(4));
    }
}
