pub struct Solution;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut last = ' ';
        let mut n = 0i64;
        let mut result = 0;
        for c in s.chars().chain([' ']) {
            if c != last {
                result = (result + n * (n + 1) / 2) % 10_0000_0007;
                last = c;
                n = 1;
            } else {
                n += 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::count_homogenous("abbcccaa".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_homogenous("xy".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(15, Solution::count_homogenous("zzzzz".to_string()));
    }
}
