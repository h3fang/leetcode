pub struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[0]);
        points.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[8, 7], [9, 9], [7, 4], [9, 7]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::max_width_of_vertical_area(points));
    }

    #[test]
    fn case2() {
        let points = [[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(3, Solution::max_width_of_vertical_area(points));
    }
}
