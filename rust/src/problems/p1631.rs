use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(mut heights: Vec<Vec<i32>>) -> i32 {
        let m = heights.len();
        let n = heights[0].len();
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), (0, 0)));
        while let Some((Reverse(d), (i, j))) = q.pop() {
            if i == m - 1 && j == n - 1 {
                return d;
            }
            let h = heights[i][j];
            if h < 0 {
                continue;
            }
            heights[i][j] = -h;

            for (i1, j1) in [
                (i + 1, j),
                (i.overflowing_sub(1).0, j),
                (i, j + 1),
                (i, j.overflowing_sub(1).0),
            ] {
                if i1 >= m || j1 >= n || heights[i1][j1] < 0 {
                    continue;
                }

                let d1 = d.max((heights[i1][j1] - h).abs());
                if d1 < dist[i1][j1] {
                    q.push((Reverse(d1), (i1, j1)));
                    dist[i1][j1] = d1;
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
        let heights = [[1, 2, 2], [3, 8, 2], [5, 3, 5]];
        let heights = heights.iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::minimum_effort_path(heights));
    }

    #[test]
    fn case2() {
        let heights = [
            [1, 2, 1, 1, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 1, 1, 2, 1],
        ];
        let heights = heights.iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::minimum_effort_path(heights));
    }
}
