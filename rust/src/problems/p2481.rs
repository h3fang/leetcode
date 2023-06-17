pub struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n % 2 == 1 {
            n
        } else {
            n / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::number_of_cuts(4));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::number_of_cuts(3));
    }
}
