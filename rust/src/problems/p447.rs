pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut m = HashMap::with_capacity(points.len());
        for a in &points {
            for b in &points {
                let d2 = (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1]);
                let e = m.entry(d2).or_insert(0);
                result += 2 * *e;
                *e += 1;
            }
            m.clear();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[0, 0], [1, 0], [2, 0]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_boomerangs(points));
    }

    #[test]
    fn case2() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_boomerangs(points));
    }

    #[test]
    fn case3() {
        let points = [[1, 1]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(0, Solution::number_of_boomerangs(points));
    }
}
