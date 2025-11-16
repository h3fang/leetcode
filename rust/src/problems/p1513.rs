pub struct Solution;

const MOD: usize = 10_0000_0007;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut ans = 0;
        for s in s.split('0') {
            let n = s.len();
            ans = (ans + (n + 1) * n / 2) % MOD;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::num_sub("0110111".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::num_sub("101".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(21, Solution::num_sub("111111".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::num_sub("000".to_string()));
    }
}
