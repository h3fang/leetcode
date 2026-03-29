pub struct Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut n = ((needed_apples / 4) as f64).cbrt() as i64;
        if 2 * n * (n + 1) * (2 * n + 1) < needed_apples {
            n += 1;
        }
        8 * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::minimum_perimeter(1));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::minimum_perimeter(13));
    }

    #[test]
    fn case3() {
        assert_eq!(5040, Solution::minimum_perimeter(1000000000));
    }
}
