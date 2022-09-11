pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut r = quality
            .iter()
            .zip(&wage)
            .enumerate()
            .map(|(i, (&q, &w))| (w as f64 / q as f64, i))
            .collect::<Vec<_>>();
        r.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut q = BinaryHeap::new();
        let mut sum = 0;
        for &(_, i) in &r[0..k as usize] {
            q.push(quality[i]);
            sum += quality[i];
        }
        let mut result = sum as f64 * r[k as usize - 1].0;
        for i in k as usize..quality.len() {
            let qu = quality[r[i].1];
            let max = *q.peek().unwrap();
            if qu < max {
                q.pop();
                sum += qu - max;
                q.push(qu);
                result = result.min(sum as f64 * r[i].0);
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
        let result = Solution::mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2);
        assert!((105.0 - result).abs() < 1e-5)
    }

    #[test]
    fn case2() {
        let result =
            Solution::mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3);
        assert!((30.66667 - result).abs() < 1e-5)
    }
}
