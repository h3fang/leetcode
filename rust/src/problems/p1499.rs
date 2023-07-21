pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = i32::MIN;
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        for p in points {
            while q.front().map_or(false, |e| p[0] - e.1 > k) {
                q.pop_front();
            }
            if let Some((a, _)) = q.front() {
                result = result.max(a + p[0] + p[1]);
            }
            while q.back().map_or(false, |e| p[1] - p[0] >= e.0) {
                q.pop_back();
            }
            q.push_back((p[1] - p[0], p[0]));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 3], [2, 0], [5, 10], [6, -10]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(4, Solution::find_max_value_of_equation(points, 1));
    }

    #[test]
    fn case2() {
        let points = [[0, 0], [3, 0], [9, 2]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(3, Solution::find_max_value_of_equation(points, 3));
    }
}
