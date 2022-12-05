pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, _p: i32, max_boxes: i32, max_weight: i32) -> i32 {
        let n = boxes.len();
        let mut w_presum = vec![0; n + 1];
        let mut neg = vec![0; n + 1];
        for (i, b) in boxes.iter().enumerate() {
            w_presum[i + 1] = w_presum[i] + b[1];
            if i > 0 {
                neg[i + 1] = neg[i] + i32::from(b[0] != boxes[i - 1][0]);
            }
        }
        let mut q = VecDeque::new();
        q.push_back(0);
        let mut f = vec![0; n + 1];
        let mut g = vec![0; n + 1];
        let max_boxes = max_boxes as usize;
        for i in 1..=n {
            while ((i - *q.front().unwrap()) > max_boxes)
                || (w_presum[i] - w_presum[*q.front().unwrap()] > max_weight)
            {
                q.pop_front();
            }
            f[i] = g[*q.front().unwrap()] + neg[i] + 2;
            if i != n {
                g[i] = f[i] - neg[i + 1];
                while !q.is_empty() && g[i] <= g[*q.back().unwrap()] {
                    q.pop_back();
                }
                q.push_back(i);
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let boxes = [[1, 1], [2, 1], [1, 1]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(4, Solution::box_delivering(boxes, 2, 3, 3));
    }

    #[test]
    fn case2() {
        let boxes = [[1, 2], [3, 3], [3, 1], [3, 1], [2, 4]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(6, Solution::box_delivering(boxes, 3, 3, 6));
    }

    #[test]
    fn case3() {
        let boxes = [[1, 4], [1, 2], [2, 1], [2, 1], [3, 2], [3, 4]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(6, Solution::box_delivering(boxes, 3, 6, 7));
    }

    #[test]
    fn case4() {
        let boxes = [
            [2, 4],
            [2, 5],
            [3, 1],
            [3, 2],
            [3, 7],
            [3, 1],
            [4, 4],
            [1, 3],
            [5, 2],
        ]
        .iter()
        .map(|b| b.to_vec())
        .collect();
        assert_eq!(14, Solution::box_delivering(boxes, 5, 5, 7));
    }
}
