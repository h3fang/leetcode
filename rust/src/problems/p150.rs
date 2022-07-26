pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut s = vec![];
        for t in tokens {
            match t.as_str() {
                "+" => {
                    let b = s.pop().unwrap();
                    let a = s.pop().unwrap();
                    s.push(a + b);
                }
                "-" => {
                    let b = s.pop().unwrap();
                    let a = s.pop().unwrap();
                    s.push(a - b);
                }
                "*" => {
                    let b = s.pop().unwrap();
                    let a = s.pop().unwrap();
                    s.push(a * b);
                }
                "/" => {
                    let b = s.pop().unwrap();
                    let a = s.pop().unwrap();
                    s.push(a / b);
                }
                x => {
                    s.push(x.parse::<i32>().unwrap());
                }
            }
        }
        s[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tokens = ["2", "1", "+", "3", "*"];
        let tokens = tokens.iter().map(|t| t.to_string()).collect();
        assert_eq!(9, Solution::eval_rpn(tokens));
    }

    #[test]
    fn case2() {
        let tokens = ["4", "13", "5", "/", "+"];
        let tokens = tokens.iter().map(|t| t.to_string()).collect();
        assert_eq!(6, Solution::eval_rpn(tokens));
    }

    #[test]
    fn case3() {
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let tokens = tokens.iter().map(|t| t.to_string()).collect();
        assert_eq!(22, Solution::eval_rpn(tokens));
    }
}
