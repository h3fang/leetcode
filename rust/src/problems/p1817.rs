pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut m: HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs {
            m.entry(log[0]).or_default().insert(log[1]);
        }
        let mut result = vec![0; k];
        for v in m.values() {
            let i = v.len() - 1;
            if i < k {
                result[i] += 1;
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
        let logs = [[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]]
            .iter()
            .map(|log| log.to_vec())
            .collect();
        assert_eq!(
            vec![0, 2, 0, 0, 0],
            Solution::finding_users_active_minutes(logs, 5)
        );
    }

    #[test]
    fn case2() {
        let logs = [[1, 1], [2, 2], [2, 3]]
            .iter()
            .map(|log| log.to_vec())
            .collect();
        assert_eq!(
            vec![1, 1, 0, 0],
            Solution::finding_users_active_minutes(logs, 4)
        );
    }
}
