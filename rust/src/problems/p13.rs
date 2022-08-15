pub struct Solution;

fn value(b: u8) -> i32 {
    match b {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut result = 0;
        for (i, &b) in s.iter().enumerate() {
            if i + 1 < n && value(b) < value(s[i + 1]) {
                result -= value(b);
            } else {
                result += value(b);
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
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
