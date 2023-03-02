pub struct Solution;

impl Solution {
    pub fn print_bin(mut num: f64) -> String {
        let mut result = String::from("0.");
        let mut f = 0.5;
        while num > 0.0 {
            if num >= f {
                result.push('1');
                num -= f;
            } else {
                result.push('0');
            }
            f /= 2.0;
            if result.len() > 32 {
                return "ERROR".to_string();
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
        assert_eq!("0.101", Solution::print_bin(0.625));
    }

    #[test]
    fn case2() {
        assert_eq!("ERROR", Solution::print_bin(0.1));
    }
}
