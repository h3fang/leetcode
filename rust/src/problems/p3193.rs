pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let mut req = vec![-1; n as usize];
        req[0] = 0;
        let mut m = 0;
        for r in requirements {
            req[r[0] as usize] = r[1];
            m = m.max(r[1]);
        }
        if req[0] > 0 {
            return 0;
        }
        let m = m as usize;
        let mut f = vec![0; m + 1];
        f[0] = 1;
        for (i, &r1) in req.iter().enumerate().skip(1) {
            let mx = if r1 == -1 { m } else { r1 as usize };
            let r = req[i - 1];
            if r >= 0 {
                let r = r as usize;
                for j in 0..m + 1 {
                    f[j] = if r <= j && j <= (i + r).min(mx) {
                        f[r]
                    } else {
                        0
                    };
                }
            } else {
                for j in 1..mx + 1 {
                    f[j] = (f[j] + f[j - 1]) % MOD;
                }
                for j in (i + 1..=mx).rev() {
                    f[j] = (f[j] + MOD - f[j - 1 - i]) % MOD;
                }
            }
        }
        f[*req.last().unwrap() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let requirements = [[2, 2], [0, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::number_of_permutations(3, requirements));
    }

    #[test]
    fn case2() {
        let requirements = [[2, 2], [1, 1], [0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::number_of_permutations(3, requirements));
    }

    #[test]
    fn case3() {
        let requirements = [[0, 0], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::number_of_permutations(2, requirements));
    }
}
