use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut g = vec![vec![]; n as usize];
        for e in &red_edges {
            g[e[0] as usize].push((e[1], 0));
        }

        for e in &blue_edges {
            g[e[0] as usize].push((e[1], 1));
        }
        let mut dist = vec![-1; n as usize];
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        visited.insert((0, 1));
        let mut q = vec![(0, 0), (0, 1)];
        let mut level = 0;
        while !q.is_empty() {
            let mut curr = vec![];
            for (x, c) in q {
                if dist[x as usize] == -1 {
                    dist[x as usize] = level;
                }
                for &(x1, c1) in &g[x as usize] {
                    if c1 != c && !visited.contains(&(x1, c1)) {
                        visited.insert((x1, c1));
                        curr.push((x1, c1));
                    }
                }
            }
            q = curr;
            level += 1;
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![0, 1, -1],
            Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 1, -1],
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![0, -1, -1],
            Solution::shortest_alternating_paths(3, vec![vec![1, 0]], vec![vec![2, 1]])
        );
    }
    #[test]
    fn case4() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![1, 2]])
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            vec![0, 1, 1],
            Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![0, 2]], vec![vec![1, 0]])
        );
    }
}
