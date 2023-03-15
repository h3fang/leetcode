pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut ranks = vec![0; n as usize];
        let mut s = HashSet::new();
        for r in &roads {
            ranks[r[0] as usize] += 1;
            ranks[r[1] as usize] += 1;
            s.insert((r[0].min(r[1]), r[0].max(r[1])));
        }
        let mut result = 0;
        for (i, &a) in ranks.iter().enumerate() {
            for (j, &b) in ranks.iter().enumerate().skip(i + 1) {
                let r = a
                    + b
                    + if s.contains(&(i as i32, j as i32)) {
                        -1
                    } else {
                        0
                    };
                result = result.max(r);
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
        let roads = [[0, 1], [0, 3], [1, 2], [1, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::maximal_network_rank(4, roads));
    }

    #[test]
    fn case2() {
        let roads = [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::maximal_network_rank(5, roads));
    }

    #[test]
    fn case3() {
        let roads = [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::maximal_network_rank(8, roads));
    }
}
