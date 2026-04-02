pub struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let n = coins[0].len();
        let mut f = vec![[i32::MIN / 2; 3]; n + 1];
        f[1] = [0; 3];
        for r in coins {
            for (j, &c) in r.iter().enumerate() {
                f[j + 1] = if c >= 0 {
                    [
                        f[j + 1][0].max(f[j][0]) + c,
                        f[j + 1][1].max(f[j][1]) + c,
                        f[j + 1][2].max(f[j][2]) + c,
                    ]
                } else {
                    [
                        f[j + 1][0].max(f[j][0]) + c,
                        f[j + 1][0]
                            .max(f[j + 1][1] + c)
                            .max(f[j][0])
                            .max(f[j][1] + c),
                        f[j + 1][1]
                            .max(f[j + 1][2] + c)
                            .max(f[j][1])
                            .max(f[j][2] + c),
                    ]
                }
            }
        }
        *f[n].iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let coins = [[0, 1, -1], [1, -2, 3], [2, -3, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(8, Solution::maximum_amount(coins));
    }

    #[test]
    fn case2() {
        let coins = [[10, 10, 10], [10, 10, 10]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(40, Solution::maximum_amount(coins));
    }
}
