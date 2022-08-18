pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut freq = HashMap::new();
        let mut max_f = 0;
        let mut result = 1;
        for (i, n) in nums.into_iter().enumerate() {
            let c = count.entry(n).or_insert(0);
            if *c > 0 {
                *freq.get_mut(&(*c)).unwrap() -= 1;
            }
            *c += 1;
            max_f = max_f.max(*c);
            let f = freq.entry(*c).or_insert(0);
            *f += 1;
            if max_f == 1
                || (freq[&max_f] == 1 && max_f + freq[&(max_f - 1)] * (max_f - 1) == i as i32 + 1)
                || (freq[&max_f] * max_f == i as i32 && freq[&1] == 1)
            {
                result = result.max(i as i32 + 1);
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
        assert_eq!(7, Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            13,
            Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5])
        );
    }
}
