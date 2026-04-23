pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::with_capacity(2 * n);
        for (i, x) in nums.into_iter().enumerate() {
            groups.entry(x).or_default().push(i);
        }

        let mut res = vec![0; n];
        for g in groups.values() {
            let total: i64 = g.iter().map(|&x| x as i64).sum();
            let mut prefix_total: i64 = 0;
            let sz = g.len() as i64;
            for (i, &idx) in g.iter().enumerate() {
                res[idx] = total - prefix_total * 2 + idx as i64 * (2 * i as i64 - sz);
                prefix_total += idx as i64;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![5, 0, 3, 4, 0], Solution::distance(vec![1, 3, 1, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0, 0, 0], Solution::distance(vec![0, 5, 3]));
    }
}
