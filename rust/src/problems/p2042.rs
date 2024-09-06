pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        for p in s.split(' ') {
            if let Ok(n) = p.parse::<u32>() {
                if n <= prev {
                    return false;
                } else {
                    prev = n;
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
        let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
        assert!(Solution::are_numbers_ascending(s));
    }

    #[test]
    fn case2() {
        let s = "hello world 5 x 5".to_string();
        assert!(!Solution::are_numbers_ascending(s));
    }

    #[test]
    fn case3() {
        let s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string();
        assert!(!Solution::are_numbers_ascending(s));
    }
}
