pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut curr = 0;
        let mut left = 0;
        let mut result = 0;
        let mut op = b'+';
        for &c in s.as_bytes().iter().chain(&[b'+']) {
            if c.is_ascii_digit() {
                curr = curr * 10 + (c - b'0') as i32;
            } else if !c.is_ascii_whitespace() {
                match op {
                    b'+' => {
                        result += left;
                        left = curr;
                    }
                    b'-' => {
                        result += left;
                        left = -curr;
                    }
                    b'*' => {
                        left *= curr;
                    }
                    b'/' => {
                        left /= curr;
                    }
                    _ => panic!(),
                }
                op = c;
                curr = 0;
            }
        }
        result + left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::calculate("3+2*2".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::calculate(" 3/2 ".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::calculate(" 3+5 / 2 ".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::calculate("0".to_string()));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::calculate("1-1+1".to_string()));
    }
}
