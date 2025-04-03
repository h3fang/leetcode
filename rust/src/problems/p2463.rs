pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();
        let factories: Vec<_> = factory
            .into_iter()
            .flat_map(|f| std::iter::repeat_n(f[0], f[1] as usize))
            .collect();
        let (m, n) = (robot.len(), factories.len());
        let mut f = vec![vec![0; n + 1]; 2];
        for i in (0..m).rev() {
            if i < m - 1 {
                f[1][n] = i64::MAX / 2;
            }
            f[0][n] = i64::MAX / 2;
            for j in (0..n).rev() {
                f[0][j] = (f[1][j + 1] + (robot[i] - factories[j]).abs() as i64).min(f[0][j + 1]);
            }
            f.swap(0, 1);
        }
        f[1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let robot = vec![0, 4, 6];
        let factory = [[2, 2], [6, 2]].iter().map(|f| f.to_vec()).collect();
        assert_eq!(4, Solution::minimum_total_distance(robot, factory));
    }

    #[test]
    fn case2() {
        let robot = vec![1, -1];
        let factory = [[-2, 1], [2, 1]].iter().map(|f| f.to_vec()).collect();
        assert_eq!(2, Solution::minimum_total_distance(robot, factory));
    }
}
