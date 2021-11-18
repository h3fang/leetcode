pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        match n {
            1 => 0,
            n => (Self::last_remaining(n - 1, m) + m) % n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::last_remaining(5, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::last_remaining(10, 17));
    }
}
