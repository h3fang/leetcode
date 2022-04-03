pub struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        fn is_valid(candies: &[i32], k: i64, amount: i32) -> bool {
            let mut count = 0;
            for &g in candies {
                count += (g / amount) as i64;
            }
            count >= k
        }
        let mut left = 1;
        let mut right = *candies.iter().max().unwrap();
        let mut result = 0;
        while left <= right {
            let mid = (left + right) / 2;
            if is_valid(&candies, k, mid) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
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
        assert_eq!(5, Solution::maximum_candies(vec![5, 8, 6], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::maximum_candies(vec![2, 5], 11));
    }
}
