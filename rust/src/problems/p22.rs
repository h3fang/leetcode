pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn bt(n: i32, left: usize, curr: &mut String, result: &mut Vec<String>) {
            if n == 0 && left == 0 {
                result.push(curr.to_string());
            } else {
                if n > 0 {
                    curr.push('(');
                    bt(n - 1, left + 1, curr, result);
                    curr.pop();
                }

                if left > 0 {
                    curr.push(')');
                    bt(n, left - 1, curr, result);
                    curr.pop();
                }
            }
        }

        let mut curr = String::new();
        let mut result = Vec::new();
        bt(n, 0, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort_unstable();
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
