pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ki = 0;
        nums.iter_mut().enumerate().for_each(|(i, e)| {
            *e = match (*e).cmp(&k) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => {
                    ki = i;
                    0
                }
                std::cmp::Ordering::Greater => 1,
            }
        });
        let mut m = HashMap::from([(0, 1)]);
        let mut sum = 0;
        let mut result = 0;
        for (i, n) in nums.into_iter().enumerate() {
            sum += n;
            if i < ki {
                *m.entry(sum).or_default() += 1;
            } else {
                let p0 = *m.entry(sum).or_default();
                let p1 = *m.entry(sum - 1).or_default();
                result += p0 + p1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_subarrays(vec![2, 3, 1], 3));
    }
}
