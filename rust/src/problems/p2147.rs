pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor = corridor.as_bytes();
        let mut f = [0, 0, 1];
        for &c in corridor.iter().rev() {
            if c == b'S' {
                f = [f[1], f[2], f[1]];
            } else {
                f = [f[0], f[1], (f[0] + f[2]) % MOD];
            }
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::number_of_ways("SSPPSPS".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::number_of_ways("PPSPSP".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::number_of_ways("S".to_string()));
    }
}
