pub struct Solution;

fn dfs(
    s: &[u8],
    i: usize,
    mut left: usize,
    mut right: usize,
    target: usize,
    curr: &mut String,
    result: &mut Vec<String>,
) {
    let n = s.len();
    if left < right || left > target || left + right + n - i < target * 2 {
        return;
    }

    if i == n {
        result.push(curr.clone());
        return;
    }

    let c = s[i];

    if c == b'(' || c == b')' {
        let mut j = i + 1;
        while j < n && s[j] == c {
            j += 1;
        }
        dfs(s, j, left, right, target, curr, result);
    }

    if c == b'(' {
        left += 1;
    } else if c == b')' {
        right += 1;
    }

    curr.push(c as char);
    dfs(s, i + 1, left, right, target, curr, result);
    curr.pop();
}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let (mut target, mut left) = (0, 0);
        for c in s.as_bytes() {
            if *c == b'(' {
                left += 1;
                target += 1;
            } else if *c == b')' && left > 0 {
                left -= 1;
            }
        }

        target -= left;

        let mut result = Vec::new();
        let mut curr = String::new();
        dfs(s.as_bytes(), 0, 0, 0, target, &mut curr, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "()())()".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["(())()", "()()()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let s = "(a)())()".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["(a())()", "(a)()()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let s = ")(".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = [""];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case4() {
        let s = "))".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = [""];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case5() {
        let s = ")()(".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
