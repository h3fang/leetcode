pub struct Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if p[0] == x || p[1] == y {
                    let d = (p[0] - x).abs() + (p[1] - y).abs();
                    Some((d, i))
                } else {
                    None
                }
            })
            .min()
            .map(|e| e.1 as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = 3;
        let y = 4;
        let points = [[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::nearest_valid_point(x, y, points));
    }

    #[test]
    fn case2() {
        let x = 3;
        let y = 4;
        let points = [[3, 4]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(0, Solution::nearest_valid_point(x, y, points));
    }

    #[test]
    fn case3() {
        let x = 3;
        let y = 4;
        let points = [[2, 3]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(-1, Solution::nearest_valid_point(x, y, points));
    }
}
