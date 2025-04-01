pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; rains.len()];
        let mut sunny = BTreeSet::new();
        let mut water = HashMap::new();
        for (i, r) in rains.into_iter().enumerate() {
            if r == 0 {
                sunny.insert(i);
            } else {
                result[i] = -1;
                if let Some(&w) = water.get(&r) {
                    if let Some(&s) = sunny.range(w..).next() {
                        result[s] = r;
                        sunny.remove(&s);
                    } else {
                        return vec![];
                    }
                }
                water.insert(r, i);
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
        assert_eq!(
            vec![-1, -1, -1, -1],
            Solution::avoid_flood(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1, 2, 1, -1, -1],
            Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1])
        );
    }

    #[test]
    fn case3() {
        assert!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]).is_empty());
    }
}
