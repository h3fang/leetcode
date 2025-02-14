pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let arr_len = arr_len as usize;
        let n = arr_len.min(steps);
        let mut f = vec![vec![0; n]; 2];
        f[0][0] = 1;
        for i in 1..=steps {
            for j in 0..n.min(steps + 1 - i) {
                f[1][j] = (f[0][j] + if j > 0 { f[0][j - 1] } else { 0 }) % MOD;
                f[1][j] = (f[1][j] + if j + 1 < n { f[0][j + 1] } else { 0 }) % MOD;
            }
            f.swap(0, 1);
        }
        f[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::num_ways(3, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::num_ways(2, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::num_ways(4, 2));
    }
}
