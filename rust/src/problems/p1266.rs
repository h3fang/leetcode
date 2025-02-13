pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|w| (w[0][0] - w[1][0]).abs().max((w[0][1] - w[1][1]).abs()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 1], [3, 4], [-1, 0]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(7, Solution::min_time_to_visit_all_points(points));
    }

    #[test]
    fn case2() {
        let points = [[3, 2], [-2, 2]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(5, Solution::min_time_to_visit_all_points(points));
    }
}
