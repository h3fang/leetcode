pub struct Solution;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (cost.len(), cost[0].len());
        let p = 1 << n;
        let mut f = vec![vec![i32::MAX / 2; p]; m + 1];
        f[0][0] = 0;
        for i in 1..=m {
            for s in 0..p {
                for k in 0..n {
                    if (s & (1 << k)) == 0 {
                        continue;
                    }
                    let c = cost[i - 1][k];
                    f[i][s] = f[i][s].min(f[i][s ^ (1 << k)] + c);
                    f[i][s] = f[i][s].min(f[i - 1][s] + c);
                    f[i][s] = f[i][s].min(f[i - 1][s ^ (1 << k)] + c);
                }
            }
        }
        f[m][p - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let cost = [[15, 96], [36, 2]].iter().map(|c| c.to_vec()).collect();
        assert_eq!(17, Solution::connect_two_groups(cost));
    }

    #[test]
    fn case2() {
        let cost = [[1, 3, 5], [4, 1, 1], [1, 5, 3]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(4, Solution::connect_two_groups(cost));
    }

    #[test]
    fn case3() {
        let cost = [[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]]
            .iter()
            .map(|c| c.to_vec())
            .collect();
        assert_eq!(10, Solution::connect_two_groups(cost));
    }
}
