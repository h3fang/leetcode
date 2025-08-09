pub struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let num = num.as_bytes();
        let n = num.len() as i32;
        let mut result = n;
        if let Some(i) = num.iter().rposition(|&x| x == b'0') {
            result = n - 1;
            if let Some(j) = num[..i].iter().rposition(|&x| x == b'0' || x == b'5') {
                result = result.min(n - j as i32 - 2);
            }
        }
        if let Some(i) = num.iter().rposition(|&x| x == b'5')
            && let Some(j) = num[..i].iter().rposition(|&x| x == b'2' || x == b'7')
        {
            result = result.min(n - j as i32 - 2);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_operations("2245047".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_operations("2908305".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_operations("10".to_string()));
    }
}
