pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut req = vec![-1; n];
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
        for i in 1..n {
            let r = req[i];
            for k in 1..=m {
                f[k] = (f[k] + f[k - 1]) % MOD;
            }
            for j in (0..=m).rev() {
                if r >= 0 && j as i32 != r {
                    f[j] = 0;
                } else if j > i.min(j) {
                    f[j] = (f[j] - f[j - i.min(j) - 1] + MOD) % MOD;
                }
            }
        }
        f[req[n - 1] as usize]
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
