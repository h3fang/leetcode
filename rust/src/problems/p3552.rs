pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_moves(matrix: Vec<String>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut portals = (0..26).map(|_| Vec::with_capacity(8)).collect::<Vec<_>>();
        for (i, r) in matrix.iter().enumerate() {
            for (j, b) in r.bytes().enumerate() {
                if !b.is_ascii_uppercase() {
                    continue;
                }
                let k = (b - b'A') as usize;
                portals[k].push((i as i32, j as i32));
            }
        }
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;
        let mut q = VecDeque::with_capacity(m * n);
        q.push_back((0, 0, 0, 0));
        let (m, n) = (m as i32, n as i32);
        while let Some((d, i, j, mask)) = q.pop_front() {
            if (i, j) == (m - 1, n - 1) {
                return d;
            }
            let cell = matrix[i as usize].as_bytes()[j as usize];
            if cell.is_ascii_uppercase() {
                let k = (cell - b'A') as usize;
                for &(i1, j1) in &portals[k] {
                    if d < dist[i1 as usize][j1 as usize] {
                        dist[i1 as usize][j1 as usize] = d;
                        q.push_front((d, i1, j1, mask | (1 << k)));
                    }
                }
                portals[k].clear();
            }
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if i1 < 0
                    || j1 < 0
                    || i1 == m
                    || j1 == n
                    || matrix[i1 as usize].as_bytes()[j1 as usize] == b'#'
                {
                    continue;
                }
                if d + 1 < dist[i1 as usize][j1 as usize] {
                    dist[i1 as usize][j1 as usize] = d + 1;
                    q.push_back((d + 1, i1, j1, mask));
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
        let matrix = ["A..", ".A.", "..."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(2, Solution::min_moves(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [".#...", ".#.#.", ".#.#.", "...#."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(13, Solution::min_moves(matrix));
    }
}
