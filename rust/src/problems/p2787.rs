pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let (n, x) = (n as usize, x as u32);
        let mut f = vec![0; n + 1];
        f[0] = 1;
        for y in 1..=n {
            let yn = y.pow(x);
            if yn > n {
                break;
            }
            for i in (yn..=n).rev() {
                f[i] = (f[i - yn] + f[i]) % MOD;
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
        assert_eq!(1, Solution::number_of_ways(10, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::number_of_ways(4, 1));
    }
}
