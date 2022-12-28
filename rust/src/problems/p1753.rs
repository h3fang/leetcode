pub struct Solution;

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let s = a + b + c;
        let max = a.max(b).max(c);
        (s - max).min(s / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::maximum_score(2, 4, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::maximum_score(4, 4, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::maximum_score(1, 8, 8));
    }
}
