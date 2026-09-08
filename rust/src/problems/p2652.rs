pub struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let s = |m| {
            let k = n / m;
            k * (k + 1) / 2 * m
        };
        s(3) + s(5) + s(7) - s(15) - s(21) - s(35) + s(105)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(21, Solution::sum_of_multiples(7));
    }

    #[test]
    fn case2() {
        assert_eq!(40, Solution::sum_of_multiples(10));
    }

    #[test]
    fn case3() {
        assert_eq!(30, Solution::sum_of_multiples(9));
    }
}
