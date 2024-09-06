pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        fn reverse(mut num: i32) -> i32 {
            let mut result = 0;
            while num > 0 {
                result = result * 10 + num % 10;
                num /= 10;
            }
            result
        }

        let mut result = 0;
        let mut cnt = HashMap::new();
        for num in nums {
            let rev = reverse(num);
            let e = cnt.entry(num - rev).or_insert(0);
            let v = *e;
            *e += 1;
            result = (result + v) % 1_000_000_007
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_nice_pairs(vec![42, 11, 1, 97]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]));
    }
}
