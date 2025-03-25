pub struct Solution;

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let (mut max_x1, mut max_x2, mut max_y1, mut max_y2) =
            (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
        let (mut min_x1, mut min_x2, mut min_y1, mut min_y2) =
            (i32::MAX, i32::MAX, i32::MAX, i32::MAX);
        let (mut min_xi, mut min_yi, mut max_xi, mut max_yi) = (0, 0, 0, 0);
        for (i, p) in points.iter().enumerate() {
            let (x, y) = (p[0] - p[1], p[0] + p[1]);
            if x < min_x1 {
                min_x2 = min_x1;
                min_x1 = x;
                min_xi = i;
            } else if x < min_x2 {
                min_x2 = x;
            }

            if x > max_x1 {
                max_x2 = max_x1;
                max_x1 = x;
                max_xi = i;
            } else if x > max_x2 {
                max_x2 = x;
            }

            if y < min_y1 {
                min_y2 = min_y1;
                min_y1 = y;
                min_yi = i;
            } else if y < min_y2 {
                min_y2 = y;
            }

            if y > max_y1 {
                max_y2 = max_y1;
                max_y1 = y;
                max_yi = i;
            } else if y > max_y2 {
                max_y2 = y;
            }
        }
        let mut result = i32::MAX;
        for i in [min_xi, max_xi, min_yi, max_yi] {
            let dx = if i == max_xi { max_x2 } else { max_x1 }
                - if i == min_xi { min_x2 } else { min_x1 };
            let dy = if i == max_yi { max_y2 } else { max_y1 }
                - if i == min_yi { min_y2 } else { min_y1 };
            result = result.min(dx.max(dy));
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
