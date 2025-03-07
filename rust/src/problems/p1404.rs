pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s.bytes().map(|b| (b - b'0') as i8).collect::<Vec<_>>();
        let mut result = 0;
        while let Some(b) = s.pop() {
            if s.is_empty() {
                break;
            }
            if b == 0 {
                result += 1;
            } else {
                let mut c = 1;
                for i in (0..s.len()).rev() {
                    if s[i] == 1 {
                        s[i] = 0;
                    } else {
                        s[i] = 1;
                        c = 0;
                        break;
                    }
                }
                if c == 1 {
                    s.insert(0, 1);
                }
                result += 2;
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
        assert_eq!(6, Solution::num_steps("1101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::num_steps("10".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_steps("1".to_string()));
    }
}
