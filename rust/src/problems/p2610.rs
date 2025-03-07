pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut m = HashMap::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut result = vec![];
        while !m.is_empty() {
            let mut r = Vec::with_capacity(m.len());
            for (&k, v) in &mut m {
                r.push(k);
                *v -= 1;
            }
            for a in &r {
                if m[a] == 0 {
                    m.remove(a);
                }
            }
            result.push(r);
        }
        result
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
        assert!(result
            .iter()
            .all(|r| r.len() == r.iter().cloned().collect::<HashSet<_>>().len()));
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
        assert!(result
            .iter()
            .all(|r| r.len() == r.iter().cloned().collect::<HashSet<_>>().len()));
    }
}
