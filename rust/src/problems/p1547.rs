pub struct Solution;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        let m = cuts.len();
        cuts.sort_unstable();
        cuts.insert(0, 0);
        cuts.push(n);
        let mut f = vec![vec![0; m + 2]; m + 2];
        for i in (1..=m).rev() {
            for j in i..=m {
                f[i][j] = if i == j { 0 } else { i32::MAX };
                for k in i..=j {
                    f[i][j] = f[i][j].min(f[i][k - 1] + f[k + 1][j]);
                }
                f[i][j] += cuts[j + 1] - cuts[i - 1];
            }
        }
        f[1][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(16, Solution::min_cost(7, vec![1, 3, 4, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(22, Solution::min_cost(9, vec![5, 6, 1, 4, 2]));
    }
}
