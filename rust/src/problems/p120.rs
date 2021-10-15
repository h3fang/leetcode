pub struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..i + 1 {
                triangle[i][j] += triangle[i + 1][j].min(triangle[i + 1][j + 1]);
            }
        }

        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(11, Solution::minimum_total(triangle));
    }

    #[test]
    fn case2() {
        let triangle = vec![vec![-10]];
        assert_eq!(-10, Solution::minimum_total(triangle));
    }
}
