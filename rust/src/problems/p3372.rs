pub struct Solution;

use std::collections::VecDeque;

fn neighbors(g: &[Vec<i32>], k: i32, x: i32) -> i32 {
    let mut q = VecDeque::with_capacity(g.len());
    q.push_back((x, 0, -1));
    let mut ans = 0;
    while let Some((x, d, p)) = q.pop_front() {
        if d > k {
            break;
        }
        ans += 1;
        for &y in &g[x as usize] {
            if y != p {
                q.push_back((y, d + 1, x));
            }
        }
    }
    ans
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let m = edges2.len() + 1;
        let mut g2 = vec![vec![]; m];
        for e in edges2 {
            g2[e[0] as usize].push(e[1]);
            g2[e[1] as usize].push(e[0]);
        }
        let max = (0..m as i32)
            .map(|x| neighbors(&g2, k - 1, x))
            .max()
            .unwrap();

        let n = edges1.len() + 1;
        let mut g1 = vec![vec![]; n];
        for e in edges1 {
            g1[e[0] as usize].push(e[1]);
            g1[e[1] as usize].push(e[0]);
        }

        (0..n as i32)
            .map(|x| {
                let c = neighbors(&g1, k, x);
                c + max
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges1 = [[0, 1], [0, 2], [2, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 2;
        assert_eq!(
            vec![9, 7, 9, 8, 8],
            Solution::max_target_nodes(edges1, edges2, k)
        );
    }

    #[test]
    fn case2() {
        let edges1 = [[0, 1], [0, 2], [0, 3], [0, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let edges2 = [[0, 1], [1, 2], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let k = 1;
        assert_eq!(
            vec![6, 3, 3, 3, 3],
            Solution::max_target_nodes(edges1, edges2, k)
        );
    }
}
