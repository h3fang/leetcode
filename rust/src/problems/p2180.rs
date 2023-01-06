pub struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (1..=num)
            .filter(|&n| {
                let mut n = n;
                let mut sum = 0;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                sum % 2 == 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_even(4));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::count_even(30));
    }
}
