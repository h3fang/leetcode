pub struct Solution;

const MOD: i32 = 10_0000_0007;

fn is_color_valid(c: i32, m: usize, pow3: &[i32]) -> bool {
    for i in 1..m {
        if (c / pow3[i]) % 3 == (c / pow3[i - 1]) % 3 {
            return false;
        }
    }
    true
}

fn is_cols_valid(c1: i32, c2: i32, pow3: &[i32]) -> bool {
    for p in pow3 {
        if (c1 / p) % 3 == (c2 / p) % 3 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut pow3 = vec![1; m];
        for i in 1..m {
            pow3[i] = pow3[i - 1] * 3;
        }
        let mut valid = Vec::with_capacity(3usize.pow(m as u32));
        for c in 0..pow3[m - 1] * 3 {
            if is_color_valid(c, m, &pow3) {
                valid.push(c);
            }
        }
        let mut next = vec![vec![]; valid.len()];
        for (i, &c1) in valid.iter().enumerate() {
            for (j, &c2) in valid.iter().enumerate() {
                if is_cols_valid(c1, c2, &pow3) {
                    next[i].push(j);
                }
            }
        }
        let mut f = vec![vec![0; valid.len()]; n];
        f[0].iter_mut().for_each(|e| *e = 1);
        for i in 1..n {
            for (j, nxt) in next.iter().enumerate().take(valid.len()) {
                for &k in nxt {
                    f[i][j] = (f[i][j] + f[i - 1][k]) % MOD;
                }
            }
        }
        f.last().unwrap().iter().fold(0, |acc, &x| (acc + x) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::color_the_grid(1, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::color_the_grid(1, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(580986, Solution::color_the_grid(5, 5));
    }
}
