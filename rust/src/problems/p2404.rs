pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for n in nums {
            if n % 2 == 0 {
                *m.entry(n).or_insert(0) += 1;
            }
        }
        let mut max_f = 0;
        let mut min_n = i32::MAX;
        for (n, f) in m.into_iter() {
            match f.cmp(&max_f) {
                std::cmp::Ordering::Greater => {
                    max_f = f;
                    min_n = n;
                }
                std::cmp::Ordering::Equal => min_n = min_n.min(n),
                _ => {}
            }
        }
        if min_n == i32::MAX {
            -1
        } else {
            min_n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]));
    }
}
