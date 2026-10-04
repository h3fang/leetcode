pub struct Solution;

fn solve(s: impl Iterator<Item = u8>, c: u8) -> i32 {
    let (mut l, mut r) = (0, 0);
    let mut ans = 0;

    for b in s {
        if b == c {
            l += 1;
        } else {
            r += 1;
        }

        if l == r {
            ans = ans.max(l * 2);
        }

        if r > l {
            r = 0;
            l = 0;
        }
    }

    ans
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        solve(s.bytes(), b'(').max(solve(s.bytes().rev(), b')'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_valid_parentheses("(()".into()));
    }
}
