pub struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (n, sum) = nums.into_iter().fold((0, 0), |(n, sum), i| {
            if i % 6 == 0 {
                (n + 1, sum + i)
            } else {
                (n, sum)
            }
        });
        if n > 0 { sum / n } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::average_value(vec![1, 3, 6, 10, 12, 15]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::average_value(vec![1, 2, 4, 7, 10]));
    }
}
