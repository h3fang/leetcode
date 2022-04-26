pub struct Solution;

// struct UnionFind {
//     parent: Vec<usize>,
//     rank: Vec<usize>,
// }

// impl UnionFind {
//     fn new(n: usize) -> Self {
//         Self {
//             parent: (0..n).collect(),
//             rank: vec![0; n],
//         }
//     }

//     fn find(&mut self, x: usize) -> usize {
//         if self.parent[x] != x {
//             self.parent[x] = self.find(self.parent[x]);
//         }
//         self.parent[x as usize]
//     }

//     fn union(&mut self, x: usize, y: usize) {
//         let px = self.find(x);
//         let py = self.find(y);
//         if px == py {
//             return;
//         }
//         match self.rank[px].cmp(&self.rank[py]) {
//             std::cmp::Ordering::Less => self.parent[py] = px,
//             std::cmp::Ordering::Equal => {
//                 self.rank[px] += 1;
//                 self.parent[py] = px;
//             }
//             std::cmp::Ordering::Greater => self.parent[px] = py,
//         }
//     }
// }

// impl Solution {
//     pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
//         let n = points.len();
//         let mut s = UnionFind::new(n);
//         let mut edges = Vec::with_capacity(n * (n - 1) / 2);
//         for (i, a) in points.iter().enumerate() {
//             for (j, b) in points.iter().enumerate().skip(i + 1) {
//                 edges.push(((a[0] - b[0]).abs() + (a[1] - b[1]).abs(), i, j));
//             }
//         }
//         edges.sort_unstable();
//         let mut result = 0;
//         for (d, x, y) in edges {
//             if s.find(x) != s.find(y) {
//                 s.union(x, y);
//                 result += d;
//             }
//         }
//         result
//     }
// }

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut q = BinaryHeap::new();
        let mut vis = vec![false; n];
        let mut dis = vec![i32::MAX; n];
        dis[0] = 0;
        q.push((Reverse(0), 0));
        let mut result = 0;
        while !q.is_empty() {
            let (Reverse(d), i) = q.pop().unwrap();
            if vis[i] {
                continue;
            }
            result += d;
            vis[i] = true;
            for j in 0..n {
                if !vis[j] {
                    let g =
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                    if g < dis[j] {
                        dis[j] = g;
                        q.push((Reverse(g), j));
                    }
                }
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
        let points = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        assert_eq!(20, Solution::min_cost_connect_points(points));
    }
}
