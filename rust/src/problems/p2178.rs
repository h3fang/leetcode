pub struct Solution;

impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        let mut result = vec![];
        if final_sum % 2 != 0 {
            return result;
        }
        let mut i = 2;
        while i <= final_sum {
            result.push(i);
            final_sum -= i;
            i += 2;
        }
        *result.last_mut().unwrap() += final_sum;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::maximum_even_split(12);
        result.sort_unstable();
        assert_eq!(3, result.len());
        assert!(result.windows(2).all(|w| w[0] != w[1]));
    }

    #[test]
    fn case2() {
        let result = Solution::maximum_even_split(7);
        assert!(result.is_empty());
    }

    #[test]
    fn case3() {
        let mut result = Solution::maximum_even_split(28);
        result.sort_unstable();
        assert_eq!(4, result.len());
        assert!(result.windows(2).all(|w| w[0] != w[1]));
    }
}
