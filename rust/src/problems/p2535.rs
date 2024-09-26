pub struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let s1: i32 = nums.iter().sum();
        let s2: i32 = nums
            .into_iter()
            .map(|mut n| {
                let mut s = 0;
                while n > 0 {
                    s += n % 10;
                    n /= 10;
                }
                s
            })
            .sum();
        (s1 - s2).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::difference_of_sum(vec![1, 15, 6, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::difference_of_sum(vec![1, 2, 3, 4]));
    }
}
