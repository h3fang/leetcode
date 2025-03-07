pub struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut f = [vec![0; n + 1], vec![i32::MAX / 2; n + 1]];
        f[1][0] = 0;
        for i in (0..n).rev() {
            for j in 1..=n {
                f[0][j] = f[1][j].min(f[1][(j as i32 - 1 - time[i]).max(0) as usize] + cost[i]);
            }
            f.swap(0, 1);
            f[0].iter_mut().for_each(|e| *e = 0);
        }
        f[1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]));
    }
}
