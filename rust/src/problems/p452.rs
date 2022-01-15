pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_unstable_by_key(|p| p[1]);
        let mut result = 1;
        let mut x = points[0][1];
        for p in points {
            if p[0] > x {
                result += 1;
                x = p[1];
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
        let points = [[10, 16], [2, 8], [1, 6], [7, 12]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        assert_eq!(2, Solution::find_min_arrow_shots(points));
    }
}
