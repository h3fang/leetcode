pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut a = 1;
        while a < bound {
            let mut b = 1;
            while a + b <= bound {
                set.insert(a + b);
                b *= y;
                if y == 1 {
                    break;
                }
            }
            a *= x;
            if x == 1 {
                break;
            }
        }
        set.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::powerful_integers(2, 3, 10);
        result.sort_unstable();
        assert_eq!(vec![2, 3, 4, 5, 7, 9, 10], result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::powerful_integers(3, 5, 15);
        result.sort_unstable();
        assert_eq!(vec![2, 4, 6, 8, 10, 14], result);
    }
}
