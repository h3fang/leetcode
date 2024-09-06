pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut primes = [false; 20];
        for p in [2, 3, 5, 7, 11, 13, 17, 19] {
            primes[p] = true;
        }
        let mut result = 0;
        for n in left..=right {
            let d = n.count_ones() as usize;
            if primes[d] {
                result += 1;
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
        assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    }
}
