pub struct Solution;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut f = vec![i32::MAX / 2; n + 1];
        f[0] = 0;
        for (&c, &t) in cost.iter().zip(&time) {
            for j in (1..=n).rev() {
                let k = (j as i32 - 1 - t).max(0) as usize;
                f[j] = f[j].min(f[k] + c);
            }
        }
        f[n]
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
