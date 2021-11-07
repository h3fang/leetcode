pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = vec![0; num1.len() + num2.len()];
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();
        for (i, &d1) in n1.iter().enumerate().rev() {
            let mut carry = 0;
            for (j, &d2) in n2.iter().enumerate().rev() {
                let n = result[i + j + 1] + (d1 - b'0') * (d2 - b'0') + carry;
                result[i + j + 1] = n % 10;
                carry = n / 10;
            }
            result[i] = carry;
        }
        let mut result: String = result.into_iter().map(|c| (c + b'0') as char).collect();
        result = result.trim_start_matches('0').to_string();
        if result.is_empty() {
            "0".into()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let num1 = "2".into();
        let num2 = "3".into();
        assert_eq!("6".to_string(), Solution::multiply(num1, num2));
    }

    #[test]
    fn case2() {
        let num1 = "123".into();
        let num2 = "456".into();
        assert_eq!("56088".to_string(), Solution::multiply(num1, num2));
    }

    #[test]
    fn case3() {
        let num1 = "123".into();
        let num2 = "0".into();
        assert_eq!("0".to_string(), Solution::multiply(num1, num2));
    }
}
