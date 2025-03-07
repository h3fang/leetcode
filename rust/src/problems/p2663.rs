pub struct Solution;

impl Solution {
    pub fn smallest_beautiful_string(mut s: String, k: i32) -> String {
        let t = unsafe { s.as_bytes_mut() };
        let last = b'a' + k as u8 - 1;
        let (i, x) = match t.iter().enumerate().rev().find_map(|(i, e)| {
            for x in *e + 1..=last {
                if i == 0 || (i == 1 && x != t[i - 1]) || (i > 1 && x != t[i - 1] && x != t[i - 2])
                {
                    return Some((i, x));
                }
            }
            None
        }) {
            Some(i) => i,
            None => return "".to_string(),
        };
        t[i] = x;
        t[i + 1..].iter_mut().for_each(|e| *e = b'a');
        for i in i + 1..t.len() {
            while (i == 1 && t[i] == t[i - 1]) || (i > 1 && (t[i] == t[i - 1] || t[i] == t[i - 2]))
            {
                t[i] += 1;
                if t[i] > last {
                    return "".to_string();
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "abda",
            Solution::smallest_beautiful_string("abcz".to_string(), 26)
        );
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::smallest_beautiful_string("dc".to_string(), 4));
    }
}
