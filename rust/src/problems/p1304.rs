pub struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        if n % 2 == 1 {
            ans.push(0);
        }
        for i in 1..=n / 2 {
            ans.push(i);
            ans.push(-i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let ans = Solution::sum_zero(5);
        assert_eq!(5, ans.len());
        assert_eq!(0, ans.into_iter().sum());
    }

    #[test]
    fn case2() {
        let ans = Solution::sum_zero(3);
        assert_eq!(3, ans.len());
        assert_eq!(0, ans.into_iter().sum());
    }

    #[test]
    fn case3() {
        let ans = Solution::sum_zero(1);
        assert_eq!(1, ans.len());
        assert_eq!(0, ans.into_iter().sum());
    }
}
