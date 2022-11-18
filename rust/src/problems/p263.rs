pub struct Solution;

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n == 0 {
            return false;
        }
        for d in [2, 3, 5] {
            while n % d == 0 {
                n /= d;
            }
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_ugly(6));
    }

    #[test]
    fn case2() {
        assert_eq!(true, Solution::is_ugly(1));
    }

    #[test]
    fn case3() {
        assert_eq!(false, Solution::is_ugly(14));
    }
}
