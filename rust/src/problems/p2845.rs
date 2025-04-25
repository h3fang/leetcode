pub struct Solution;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let (mut ans, mut presum) = (0, 0);
        let mut m = vec![0; (nums.len() + 1).min(modulo as usize)];
        m[0] = 1;
        for x in nums {
            presum += i32::from(x % modulo == k);
            if presum >= k {
                ans += m[((presum - k) % modulo) as usize];
            }
            m[(presum % modulo) as usize] += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0)
        );
    }
}
