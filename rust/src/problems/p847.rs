use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len() as u8;
        let mut seen: HashSet<(u8, u16)> = HashSet::new();
        let mut q = VecDeque::new();
        for i in 0..n {
            q.push_back((i, 1 << i, 0));
            seen.insert((i, 1 << i));
        }
        let end = (1 << n) - 1;
        while let Some((i, mask, dist)) = q.pop_front() {
            if mask == end {
                return dist;
            }
            for &next in &graph[i as usize] {
                let next = next as u8;
                let mask_next = mask | (1 << next);
                if !seen.contains(&(next, mask_next)) {
                    q.push_back((next, mask_next, dist + 1));
                    seen.insert((next, mask_next));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        assert_eq!(4, Solution::shortest_path_length(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
        assert_eq!(4, Solution::shortest_path_length(graph));
    }
}
