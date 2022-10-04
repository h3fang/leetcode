pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for &b in s.as_bytes() {
            if b == b'(' {
                left += 1;
            } else if left > 0 {
                left -= 1;
            } else {
                right += 1;
            }
        }
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_add_to_make_valid("())".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_add_to_make_valid("(((".to_string()));
    }
}
