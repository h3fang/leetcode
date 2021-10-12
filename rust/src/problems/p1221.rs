pub struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut r = 0;
        let mut n = 0;
        for c in s.chars() {
            if c == 'L' {
                n += 1;
            } else {
                n -= 1;
            }

            if n == 0 {
                r += 1;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::balanced_string_split("RLRRLLRLRL".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::balanced_string_split("RLLLLRRRLR".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::balanced_string_split("LLLLRRRR".to_string()));
    }
}
