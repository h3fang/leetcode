pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut in_deg = vec![0; n as usize];
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            in_deg[e[1] as usize] += 1;
        }
        let mut q = VecDeque::new();
        for (i, &d) in in_deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        let mut ancestor = vec![HashSet::new(); n as usize];
        while let Some(i) = q.pop_front() {
            for &j in &g[i] {
                ancestor[j as usize].insert(i);
                let aj = &mut ancestor[j as usize] as *mut HashSet<_>;
                unsafe {
                    (*aj).extend(&ancestor[i]);
                }
                in_deg[j as usize] -= 1;
                if in_deg[j as usize] == 0 {
                    q.push_back(j as usize);
                }
            }
        }
        ancestor
            .into_iter()
            .map(|mut a| {
                let mut v = a.drain().map(|e| e as i32).collect::<Vec<_>>();
                v.sort_unstable();
                v
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [
            [0, 3],
            [0, 4],
            [1, 3],
            [2, 4],
            [2, 7],
            [3, 5],
            [3, 6],
            [3, 7],
            [4, 6],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let expected = vec![
            vec![],
            vec![],
            vec![],
            vec![0, 1],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        assert_eq!(expected, Solution::get_ancestors(8, edges));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 4],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let expected = vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]];
        assert_eq!(expected, Solution::get_ancestors(5, edges));
    }
}
