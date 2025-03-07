pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let (mut xs, mut ys) = (BTreeMap::new(), BTreeMap::new());
        for p in &points {
            *xs.entry(p[0] - p[1]).or_insert(0) += 1;
            *ys.entry(p[0] + p[1]).or_insert(0) += 1;
        }
        let mut result = i32::MAX;
        for p in &points {
            let (x, y) = (p[0] - p[1], p[0] + p[1]);
            let e = xs.entry(x).or_default();
            *e -= 1;
            if *e == 0 {
                xs.remove(&x);
            }
            let e = ys.entry(y).or_default();
            *e -= 1;
            if *e == 0 {
                ys.remove(&y);
            }
            if !xs.is_empty() && !ys.is_empty() {
                let dx = xs.keys().last().unwrap() - xs.keys().next().unwrap();
                let dy = ys.keys().last().unwrap() - ys.keys().next().unwrap();
                result = result.min(dx.max(dy));
            }
            *xs.entry(x).or_default() += 1;
            *ys.entry(y).or_default() += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[3, 10], [5, 15], [10, 2], [4, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(12, Solution::minimum_distance(points));
    }

    #[test]
    fn case2() {
        let points = [[1, 1], [1, 1], [1, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(0, Solution::minimum_distance(points));
    }
}
