pub struct Solution;

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let plus_pos = expression
            .char_indices()
            .find_map(|(i, c)| if c == '+' { Some(i) } else { None })
            .unwrap();
        let (mut left, mut right) = (0, expression.len() - 1);
        let a = expression[..plus_pos].parse::<i32>().unwrap();
        let b = expression[plus_pos + 1..].parse::<i32>().unwrap();
        let mut min = a + b;
        let mut result = expression.clone();
        let expression = expression.as_bytes();
        let mut n1 = 0;
        let mut n2 = a;
        for i in 0..plus_pos {
            if i > 0 {
                let d1 = (expression[i - 1] - b'0') as i32;
                n1 = n1 * 10 + d1;
                n2 -= d1 * 10i32.pow((plus_pos - i) as u32);
            }

            let mut n3 = b;
            let mut n4 = 0;
            for j in (plus_pos + 1..expression.len()).rev() {
                if j + 1 < expression.len() {
                    let d2 = (expression[j + 1] - b'0') as i32;
                    n3 /= 10;
                    n4 += d2 * 10i32.pow((expression.len() - j - 2) as u32);
                }
                let mut d = n2 + n3;
                if n1 > 0 {
                    d *= n1;
                }
                if n4 > 0 {
                    d *= n4;
                }
                if d < min {
                    min = d;
                    left = i;
                    right = j;
                }
            }
        }
        result.insert(left, '(');
        result.insert(right + 2, ')');
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("2(47+38)", Solution::minimize_result("247+38".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("1(2+3)4", Solution::minimize_result("12+34".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "(999+999)",
            Solution::minimize_result("999+999".to_string())
        );
    }
}
