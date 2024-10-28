pub struct Solution;

impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mask = (1 << n) - 1;
        let mut result = Vec::with_capacity(1 << n);
        for x in 0..1 << n {
            if x & (x >> 1) == 0 {
                result.push(format!("{:01$b}", x ^ mask, n as usize));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::valid_strings(3);
        result.sort_unstable();
        let mut expected = ["010", "011", "101", "110", "111"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::valid_strings(1);
        result.sort_unstable();
        let mut expected = ["0", "1"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
