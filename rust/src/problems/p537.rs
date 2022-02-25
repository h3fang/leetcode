pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        fn parse(num: &str) -> (i32, i32) {
            let (p1, p2) = num[..num.len() - 1].split_once('+').unwrap();
            let a = p1.parse().unwrap();
            let b = p2.parse().unwrap();
            (a, b)
        }
        let n1 = parse(&num1);
        let n2 = parse(&num2);
        let a = n1.0 * n2.0 - n1.1 * n2.1;
        let b = n1.0 * n2.1 + n1.1 * n2.0;
        format!("{a}+{b}i")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "0+2i",
            Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "0+-2i",
            Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string())
        );
    }
}
