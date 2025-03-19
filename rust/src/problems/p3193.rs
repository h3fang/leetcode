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
        let n = n as usize;
        let mut f = vec![vec![0; m + 1]; n];
        f.iter_mut().for_each(|x| x[0] = 1);
        for i in 1..n {
            let r = req[i];
            for j in 1..=m {
                if r >= 0 && j as i32 != r {
                    continue;
                }
                for k in 0..=j {
                    f[i][j] = (f[i][j] + f[i - 1][k]) % MOD;
                }
            }
        }
        f[n - 1].iter().fold(0, |acc, x| (acc + x) % MOD)
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
