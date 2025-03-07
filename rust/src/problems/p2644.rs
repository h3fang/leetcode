pub struct Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        divisors
            .into_iter()
            .map(|d| {
                let c = nums.iter().filter(|&&n| n % d == 0).count();
                (c, -d)
            })
            .max()
            .map(|e| -e.1)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::max_div_score(vec![20, 14, 21, 10], vec![5, 7, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(10, Solution::max_div_score(vec![12], vec![10, 16]));
    }
}
