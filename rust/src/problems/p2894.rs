pub struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let sum = (1 + n) * n / 2;
        let k = n / m;
        sum - k * (k + 1) * m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(19, Solution::difference_of_sums(10, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::difference_of_sums(5, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(-15, Solution::difference_of_sums(5, 1));
    }
}
