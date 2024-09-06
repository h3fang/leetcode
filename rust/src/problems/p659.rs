pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();
        for &n in &nums {
            *count.entry(n).or_insert(0) += 1;
        }
        let mut seqs = HashMap::new();
        for n in nums {
            if *count.get(&n).unwrap() == 0 {
                continue;
            }
            if let Some(f) = seqs.get_mut(&(n - 1)) {
                *count.get_mut(&n).unwrap() -= 1;
                *f -= 1;
                if *f == 0 {
                    seqs.remove(&(n - 1));
                }
                *seqs.entry(n).or_insert(0) += 1;
            } else {
                let c1 = *count.get(&(n + 1)).unwrap_or(&0);
                let c2 = *count.get(&(n + 2)).unwrap_or(&0);
                if c1 > 0 && c2 > 0 {
                    *count.get_mut(&n).unwrap() -= 1;
                    *count.get_mut(&(n + 1)).unwrap() -= 1;
                    *count.get_mut(&(n + 2)).unwrap() -= 1;
                    *seqs.entry(n + 2).or_insert(0) += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_possible(vec![1, 2, 3, 4, 4, 5]));
    }
}
