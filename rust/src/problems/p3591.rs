pub struct Solution;

fn is_prime(x: i32) -> bool {
    let mut y = 2;
    while y * y <= x {
        if x % y == 0 {
            return false;
        }
        y += 1;
    }
    x > 1
}

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut f = [0; 101];
        for x in nums {
            f[x as usize] += 1;
        }
        f.into_iter().any(is_prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case3() {
        assert!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]));
    }
}
