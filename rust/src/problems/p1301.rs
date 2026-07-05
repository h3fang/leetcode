pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let mut f = vec![(i32::MIN, 0); n + 1];
        f[0] = (0, 1);
        for row in board {
            let mut pre = f[0];
            f[0] = (i32::MIN, 0);

            for (j, &b) in row.as_bytes().iter().enumerate() {
                if b == b'X' {
                    pre = f[j + 1];
                    f[j + 1] = (i32::MIN, 0);
                    continue;
                }

                let tmp = f[j + 1];

                let max = pre.0.max(f[j + 1].0).max(f[j].0);

                let mut ways = 0i64;

                if max == pre.0 {
                    ways += pre.1 as i64;
                }

                if max == f[j].0 {
                    ways += f[j].1 as i64;
                }

                if max == f[j + 1].0 {
                    ways += f[j + 1].1 as i64;
                }

                f[j + 1].0 = max
                    + if b.is_ascii_digit() {
                        (b - b'0') as i32
                    } else {
                        0
                    };
                f[j + 1].1 = (ways % MOD) as i32;

                pre = tmp;
            }
        }

        if f[n].0 < 0 {
            vec![0, 0]
        } else {
            vec![f[n].0, f[n].1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = ["E23", "2X2", "12S"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(vec![7, 1], Solution::paths_with_max_score(board));
    }

    #[test]
    fn case2() {
        let board = ["E12", "1X1", "21S"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(vec![4, 2], Solution::paths_with_max_score(board));
    }

    #[test]
    fn case3() {
        let board = ["E11", "XXX", "11S"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(vec![0, 0], Solution::paths_with_max_score(board));
    }
}
