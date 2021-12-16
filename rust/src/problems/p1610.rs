use std::f64::consts::PI;

pub struct Solution;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let n = points.len();
        let mut angles = points
            .iter()
            .filter_map(|p| {
                let x = p[0] - location[0];
                let y = p[1] - location[1];
                if x == 0 && y == 0 {
                    None
                } else {
                    Some(f64::atan2(y as f64, x as f64))
                }
            })
            .collect::<Vec<_>>();
        let m = angles.len();
        let same = (n - m) as i32;
        angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..m {
            angles.push(angles[i] + 2.0 * PI);
        }
        let mut max = 0;
        let mut right = 0;
        let angle = (angle as f64).to_radians();
        for (left, lb) in angles.iter().enumerate() {
            let ub = lb + angle;
            while right < angles.len() && angles[right] <= ub {
                right += 1;
            }
            max = max.max(right - left);
        }
        same + max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[2, 1], [2, 2], [3, 3]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        let angle = 90;
        let location = vec![1, 1];
        assert_eq!(3, Solution::visible_points(points, angle, location));
    }

    #[test]
    fn case2() {
        let points = [[2, 1], [2, 2], [3, 4], [1, 1]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        let angle = 90;
        let location = vec![1, 1];
        assert_eq!(4, Solution::visible_points(points, angle, location));
    }

    #[test]
    fn case3() {
        let points = [[1, 0], [2, 1]];
        let points = points.iter().map(|p| p.to_vec()).collect();
        let angle = 13;
        let location = vec![1, 1];
        assert_eq!(1, Solution::visible_points(points, angle, location));
    }
}
