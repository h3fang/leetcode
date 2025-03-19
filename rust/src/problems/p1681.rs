use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let m = 1 << n;
        let mut f = vec![i32::MAX; m];
        f[0] = 0;
        let g = n / k as usize;
        let mut values = HashMap::new();
        for mask in 1..m as i32 {
            if mask.count_ones() != g as u32 {
                continue;
            }
            let mut min = 20;
            let mut max = 0;
            let mut cur = HashSet::new();
            for (i, &e) in nums.iter().enumerate() {
                if (mask & (1 << i)) > 0 {
                    if cur.contains(&e) {
                        break;
                    }
                    min = min.min(e);
                    max = max.max(e);
                    cur.insert(e);
                }
            }
            if cur.len() == g {
                values.insert(mask, max - min);
            }
        }
        for mask in 0..m {
            if f[mask] == i32::MAX {
                continue;
            }
            let mut seen = HashMap::new();
            for (i, &e) in nums.iter().enumerate() {
                if (mask & (1 << i)) == 0 {
                    seen.insert(e, i);
                }
            }
            if seen.len() < g {
                continue;
            }
            let mut sub = 0;
            for i in seen.values() {
                sub |= 1 << i;
            }
            let mut next = sub;
            while next > 0 {
                if let Some(&v) = values.get(&(next as i32)) {
                    f[mask | next] = f[mask | next].min(f[mask] + v);
                }
                next = (next - 1) & sub;
            }
        }
        if f[m - 1] < i32::MAX { f[m - 1] } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::minimum_incompatibility(vec![1, 2, 1, 4], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3)
        );
    }
}
