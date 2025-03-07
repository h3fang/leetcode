pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let routes: Vec<HashSet<i32>> = routes
            .into_iter()
            .map(|r| r.into_iter().collect())
            .collect();
        let mut g = vec![vec![]; routes.len()];
        for (i, a) in routes.iter().enumerate() {
            for (j, b) in routes.iter().enumerate().skip(i + 1) {
                if a.iter().any(|x| b.contains(x)) {
                    g[i].push(j);
                    g[j].push(i);
                }
            }
        }
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        routes.iter().enumerate().for_each(|(i, r)| {
            if r.contains(&source) {
                q.push_back((1, i));
                visited.insert(i);
            }
        });
        while let Some((cost, i)) = q.pop_front() {
            if routes[i].contains(&target) {
                return cost;
            }
            for &j in &g[i] {
                if !visited.contains(&j) {
                    visited.insert(j);
                    q.push_back((cost + 1, j));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(2, Solution::num_buses_to_destination(routes, 1, 6));
    }

    #[test]
    fn case2() {
        let routes = vec![
            vec![7, 12],
            vec![4, 5, 15],
            vec![6],
            vec![15, 19],
            vec![9, 12, 13],
        ];
        assert_eq!(-1, Solution::num_buses_to_destination(routes, 15, 12));
    }
}
