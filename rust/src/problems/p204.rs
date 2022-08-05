pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut m = vec![true; n];
        let mut result = 0;
        for i in 2..n {
            if m[i] {
                result += 1;
                if (i * i) >= n {
                    continue;
                }
                for j in (i * i..n).step_by(i) {
                    m[j] = false;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_primes(10));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_primes(0));
    }
}
