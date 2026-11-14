pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans = i32::MAX;
        let mut s = vec![0; nums.len() + 1];
        let mut map = BTreeMap::new();

        for (j, &x) in nums.iter().enumerate() {
            s[j + 1] = s[j] + x;
            if j + 1 < l as usize {
                continue;
            }
            *map.entry(s[j + 1 - l as usize]).or_insert(0) += 1;
            if let Some((sum, _)) = map.range(..s[j + 1]).last() {
                ans = ans.min(s[j + 1] - sum);
            }
            if j + 1 >= r as usize {
                let e = map.entry(s[j + 1 - r as usize]).or_default();
                *e -= 1;
                if *e == 0 {
                    map.remove(&s[j + 1 - r as usize]);
                }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4));
    }
}
