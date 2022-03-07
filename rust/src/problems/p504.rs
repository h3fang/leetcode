pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut result = String::new();
        let sign = num.signum();
        let mut num = num.abs();
        while num > 0 {
            let d = num % 7;
            result.push((d as u8 + b'0') as char);
            num /= 7;
        }
        if result.is_empty() {
            result.push('0');
        }
        if sign == -1 {
            result.push('-');
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("202", Solution::convert_to_base7(100));
    }

    #[test]
    fn case2() {
        assert_eq!("-10", Solution::convert_to_base7(-7));
    }
}
