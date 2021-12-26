use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut q = BinaryHeap::new();
        let k = k as usize;
        for (i, p) in points.iter().enumerate() {
            let dist = p[0] * p[0] + p[1] * p[1];
            if q.len() < k {
                q.push((dist, i));
            } else {
                let max = q.peek().unwrap().0;
                if dist < max {
                    q.pop();
                    q.push((dist, i));
                }
            }
        }
        q.into_iter().map(|(_, i)| points[i].clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[3, 3], [5, -1], [-2, 4]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        let k = 2;
        let mut result = Solution::k_closest(points, k);
        result.sort_unstable();
        let mut expected = vec![vec![3, 3], vec![-2, 4]];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
