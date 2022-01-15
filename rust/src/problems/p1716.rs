pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let k = n / 7;
        let r = n % 7;
        (28 + 28 + 7 * (k - 1)) * k / 2 + (k + 1 + k + 1 + r - 1) * r / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(96, Solution::total_money(20));
    }
}
