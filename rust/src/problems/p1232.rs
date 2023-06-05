pub struct Solution;

impl Solution {
    pub fn check_straight_line(cs: Vec<Vec<i32>>) -> bool {
        let n = cs.len();
        if n == 2 {
            return true;
        }
        let dx0 = cs[1][0] - cs[0][0];
        let dy0 = cs[1][1] - cs[0][1];
        for i in 2..n {
            let dxi = cs[i][0] - cs[0][0];
            let dyi = cs[i][1] - cs[0][1];
            if dx0 * dyi != dy0 * dxi {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let coordinates = [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert!(Solution::check_straight_line(coordinates));
    }

    #[test]
    fn case2() {
        let coordinates = [[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert!(!Solution::check_straight_line(coordinates));
    }
}
