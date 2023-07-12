pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; graph.len()];
        for (i, e) in graph.iter().enumerate() {
            for &j in e {
                g[j as usize].push(i);
            }
        }
        let mut indeg = graph.iter().map(|e| e.len()).collect::<Vec<_>>();
        let mut q = VecDeque::new();
        for (i, &d) in indeg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        while let Some(i) = q.pop_front() {
            for &j in &g[i] {
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    q.push_back(j);
                }
            }
        }
        indeg
            .into_iter()
            .enumerate()
            .filter_map(|(i, d)| if d == 0 { Some(i as i32) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        assert_eq!(vec![2, 4, 5, 6], Solution::eventual_safe_nodes(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        assert_eq!(vec![4], Solution::eventual_safe_nodes(graph));
    }
}
