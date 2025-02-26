pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut odd, mut even, mut sum, mut result) = (0, 1, 0, 0);
        for x in arr {
            sum ^= x & 1;
            if sum == 0 {
                even += 1;
                result = (result + odd) % MOD;
            } else {
                odd += 1;
                result = (result + even) % MOD;
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
        assert_eq!(4, Solution::num_of_subarrays(vec![1, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::num_of_subarrays(vec![2, 4, 6]));
    }

    #[test]
    fn case3() {
        assert_eq!(16, Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]));
    }
}
