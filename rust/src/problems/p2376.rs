pub struct Solution;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let digits = n.to_string().into_bytes();
        let mut dp = vec![vec![-1; 1 << 10]; digits.len()];
        fn dfs(
            dp: &mut [Vec<i32>],
            digits: &[u8],
            i: usize,
            mask: u16,
            is_smaller: bool,
            is_start: bool,
        ) -> i32 {
            if i == dp.len() {
                return i32::from(!is_start);
            }
            if is_smaller && !is_start && dp[i][mask as usize] >= 0 {
                return dp[i][mask as usize];
            }
            let mut result = 0;
            if is_start {
                result = dfs(dp, digits, i + 1, mask, true, true);
            }
            let lb = u8::from(is_start);
            let digit = digits[i] - b'0';
            let ub = if is_smaller { 9 } else { digit };
            for d in lb..=ub {
                if mask & (1 << d) == 0 {
                    result += dfs(
                        dp,
                        digits,
                        i + 1,
                        mask | (1 << d),
                        is_smaller || d < digit,
                        false,
                    );
                }
            }
            if is_smaller && !is_start {
                dp[i][mask as usize] = result;
            }
            result
        }
        dfs(&mut dp, &digits, 0, 0, false, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(19, Solution::count_special_numbers(20));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::count_special_numbers(5));
    }

    #[test]
    fn case3() {
        assert_eq!(110, Solution::count_special_numbers(135));
    }
}
