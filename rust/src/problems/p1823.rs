pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut p = 0;
        for i in 2..=n {
            p = (p + k) % i;
        }
        p + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::find_the_winner(5, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_the_winner(6, 5));
    }
}
