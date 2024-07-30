pub struct Solution;

impl Solution {
    pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
        points.sort_unstable();
        let (mut result, mut right) = (0, -1);
        for p in points {
            if p[0] > right {
                right = p[0] + w;
                result += 1;
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
        let points = [[2, 1], [1, 0], [1, 4], [1, 8], [3, 5], [4, 6]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::min_rectangles_to_cover_points(points, 1));
    }

    #[test]
    fn case2() {
        let points = [[0, 0], [1, 1], [2, 2], [3, 3], [4, 4], [5, 5], [6, 6]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(3, Solution::min_rectangles_to_cover_points(points, 2));
    }

    #[test]
    fn case3() {
        let points = [[2, 3], [1, 2]].iter().map(|p| p.to_vec()).collect();
        assert_eq!(2, Solution::min_rectangles_to_cover_points(points, 0));
    }
}
