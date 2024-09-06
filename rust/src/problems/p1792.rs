pub struct Solution;

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Ratio {
    pass: i32,
    total: i32,
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = (other.total as i64 + 1) * other.total as i64 * (self.total - self.pass) as i64;
        let b = (self.total as i64 + 1) * self.total as i64 * (other.total - other.pass) as i64;
        a.cmp(&b)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let n = classes.len();
        let mut q: BinaryHeap<_> = classes
            .into_iter()
            .map(|c| Ratio {
                pass: c[0],
                total: c[1],
            })
            .collect();
        for _ in 0..extra_students {
            if let Some(r) = q.pop() {
                q.push(Ratio {
                    pass: r.pass + 1,
                    total: r.total + 1,
                });
            }
        }
        let mut result = 0.0;
        while let Some(r) = q.pop() {
            result += r.pass as f64 / r.total as f64;
        }
        result / n as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {a}, b = {b}");
    }

    #[test]
    fn case1() {
        let classes = [[1, 2], [3, 5], [2, 2]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_close(0.78333, Solution::max_average_ratio(classes, 2));
    }

    #[test]
    fn case2() {
        let classes = [[2, 4], [3, 9], [4, 5], [2, 10]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_close(0.53485, Solution::max_average_ratio(classes, 4));
    }
}
