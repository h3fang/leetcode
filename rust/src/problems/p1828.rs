pub struct Solution;

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .into_iter()
            .map(|q| {
                let r2 = q[2] * q[2];
                points
                    .iter()
                    .filter(|p| (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]) <= r2)
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let points = [[1, 3], [3, 3], [5, 3], [2, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let queries = [[2, 3, 1], [4, 3, 1], [1, 1, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(vec![3, 2, 2], Solution::count_points(points, queries));
    }

    #[test]
    fn case2() {
        let points = [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let queries = [[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(vec![2, 3, 2, 4], Solution::count_points(points, queries));
    }
}
