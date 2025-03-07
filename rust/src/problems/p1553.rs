pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        fn f(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            if n <= 1 {
                return n;
            }
            if let Some(&r) = cache.get(&n) {
                return r;
            }
            let r = 1 + (n % 2 + f(n / 2, cache)).min(n % 3 + f(n / 3, cache));
            cache.insert(n, r);
            r
        }
        f(n, &mut Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_days(10));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_days(6));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::min_days(1));
    }
}
