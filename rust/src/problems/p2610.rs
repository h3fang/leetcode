pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freq = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut ans = vec![];
        let n = freq.len();
        for (num, cnt) in freq {
            let cnt = cnt as usize;
            if cnt > ans.len() {
                ans.resize(cnt, Vec::with_capacity(n));
            }
            ans.iter_mut().take(cnt).for_each(|v| v.push(num));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 3, 4, 1, 2, 3, 1];
        let result = Solution::find_matrix(nums.clone());
        assert_eq!(3, result.len());
        let mut r = result.iter().flatten().cloned().collect::<Vec<_>>();
        r.sort_unstable();
        nums.sort_unstable();
        assert_eq!(r, nums);
        assert!(
            result
                .iter()
                .all(|r| r.len() == r.iter().cloned().collect::<HashSet<_>>().len())
        );
    }

    #[test]
    fn case2() {
        let mut nums = vec![1, 2, 3, 4];
        let result = Solution::find_matrix(nums.clone());
        assert_eq!(1, result.len());
        let mut r = result.iter().flatten().cloned().collect::<Vec<_>>();
        r.sort_unstable();
        nums.sort_unstable();
        assert_eq!(r, nums);
        assert!(
            result
                .iter()
                .all(|r| r.len() == r.iter().cloned().collect::<HashSet<_>>().len())
        );
    }
}
