pub struct Solution;

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let (mut result, mut rev) = (0, 0);
        for i in 0..n {
            if i >= k && nums[i - k] > 1 {
                rev ^= 1;
            }
            if nums[i] == rev {
                if i + k > n {
                    return -1;
                }
                result += 1;
                rev ^= 1;
                nums[i] += 2;
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
        assert_eq!(2, Solution::min_k_bit_flips(vec![0, 1, 0], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_k_bit_flips(vec![1, 1, 0], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3)
        );
    }
}
