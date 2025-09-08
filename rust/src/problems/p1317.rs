pub struct Solution;

fn is_non_zero_integer(mut x: i32) -> bool {
    if x <= 0 {
        return false;
    }
    while x > 0 {
        if x % 10 == 0 {
            return false;
        }
        x /= 10;
    }
    true
}

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for x in 1..n {
            if is_non_zero_integer(x) && is_non_zero_integer(n - x) {
                return vec![x, n - x];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: i32) {
        let r = Solution::get_no_zero_integers(n);
        assert_eq!(n, r[0] + r[1]);
        assert!(is_non_zero_integer(r[0]));
        assert!(is_non_zero_integer(r[1]));
    }

    #[test]
    fn case1() {
        check(2);
    }

    #[test]
    fn case2() {
        check(11);
    }

    #[test]
    fn case3() {
        check(10000);
    }

    #[test]
    fn case4() {
        check(69);
    }

    #[test]
    fn case5() {
        check(1010);
    }
}
