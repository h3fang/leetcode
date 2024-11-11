pub struct Solution;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let mut primes = Vec::with_capacity(1000);
        primes.push(0);
        let mut is_prime = [true; 1001];
        for i in 2..1001 {
            if is_prime[i] {
                primes.push(i as i32);
            }
            for j in ((i * i)..1001).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut pre = 0;
        for x in nums {
            if x <= pre {
                return false;
            }
            let i = primes.partition_point(|&p| p < x - pre);
            pre = x - primes[i - 1];
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::prime_sub_operation(vec![4, 9, 6, 10]));
    }

    #[test]
    fn case2() {
        assert!(Solution::prime_sub_operation(vec![6, 8, 11, 12]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::prime_sub_operation(vec![5, 8, 3]));
    }
}
