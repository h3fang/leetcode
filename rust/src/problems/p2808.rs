pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            m.entry(x).or_default().push(i);
        }
        m.values()
            .map(|v| {
                let r = v.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0);
                (r.max(v[0] + n - *v.last().unwrap()) / 2) as i32
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_seconds(vec![1, 2, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_seconds(vec![2, 1, 3, 3, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_seconds(vec![5, 5, 5, 5]));
    }
}
