pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut st = Vec::with_capacity(n);
        let mut pair = vec![0; n];
        for (i, &b) in s.iter().enumerate() {
            match b {
                b'(' => {
                    st.push(i);
                }
                b')' => {
                    let j = st.pop().unwrap();
                    pair[i] = j as i32;
                    pair[j] = i as i32;
                }
                _ => {}
            }
        }
        let mut result = String::with_capacity(s.len());
        let (mut i, mut di) = (0, 1);
        while i < n as i32 {
            match s[i as usize] {
                b'(' | b')' => {
                    i = pair[i as usize];
                    di = -di;
                }
                x => result.push(x as char),
            }
            i += di;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("dcba", Solution::reverse_parentheses("(abcd)".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "iloveu",
            Solution::reverse_parentheses("(u(love)i)".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "leetcode",
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string())
        );
    }
}
