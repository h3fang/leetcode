pub struct Solution;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let dx1 = points[1][0] - points[0][0];
        let dy1 = points[1][1] - points[0][1];
        let dx2 = points[2][0] - points[1][0];
        let dy2 = points[2][1] - points[1][1];
        dy1 * dx2 != dx1 * dy2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 3],
            vec![3, 2]
        ]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3]
        ]));
    }
}
