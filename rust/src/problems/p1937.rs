pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points[0].len();
        let mut f = vec![vec![0; n]; 2];
        for r in points {
            let mut max = i64::MIN;
            for (j, &x) in r.iter().enumerate() {
                max = max.max(f[0][j] + j as i64);
                f[1][j] = max + x as i64 - j as i64;
            }
            max = i64::MIN;
            for (j, &x) in r.iter().enumerate().rev() {
                max = max.max(f[0][j] - j as i64);
                f[1][j] = f[1][j].max(max + x as i64 + j as i64);
            }
            f.swap(0, 1);
        }
        *f[0].iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 2, 3], [1, 5, 1], [3, 1, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(9, Solution::max_points(points));
    }

    #[test]
    fn case2() {
        let points = [[1, 5], [2, 3], [4, 2]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(11, Solution::max_points(points));
    }
}
