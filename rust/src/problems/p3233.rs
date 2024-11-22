pub struct Solution;

fn sqrt(x: i32) -> usize {
    (x as f64).sqrt() as usize
}

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = sqrt(r) + 1;
        let mut primes = vec![0; n];
        for i in 2..n {
            if primes[i] == 0 {
                primes[i] = primes[i - 1] + 1;
                for j in (i * i..n).step_by(i) {
                    primes[j] = 1;
                }
            } else {
                primes[i] = primes[i - 1];
            }
        }
        r - l + 1 - (primes[n - 1] - primes[sqrt(l - 1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::non_special_count(5, 7));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::non_special_count(4, 16));
    }
}
