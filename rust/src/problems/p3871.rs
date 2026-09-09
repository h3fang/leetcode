pub struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut group = 1000;
        let mut ans = 0;

        while group <= n {
            ans += n - group + 1;
            group *= 1000;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_commas(1002));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_commas(998));
    }
}
