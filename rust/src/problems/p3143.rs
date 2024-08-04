pub struct Solution;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let s = s.as_bytes();
        let mut min = [i32::MAX; 26];
        let mut min2 = i32::MAX;
        for (i, p) in points.into_iter().enumerate() {
            let d = p[0].abs().max(p[1].abs());
            let j = (s[i] - b'a') as usize;
            if d < min[j] {
                min2 = min2.min(min[j]);
                min[j] = d;
            } else {
                min2 = min2.min(d);
            }
        }
        min.into_iter().filter(|&d| d < min2).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[2, 2], [-1, -2], [-4, 4], [-3, 1], [3, -3]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(
            2,
            Solution::max_points_inside_square(points, "abdca".to_string())
        );
    }

    #[test]
    fn case2() {
        let points = [[1, 1], [-2, -2], [-2, 2]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(
            1,
            Solution::max_points_inside_square(points, "abb".to_string())
        );
    }

    #[test]
    fn case3() {
        let points = [[1, 1], [-1, -1], [2, -2]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(
            0,
            Solution::max_points_inside_square(points, "ccd".to_string())
        );
    }
}
