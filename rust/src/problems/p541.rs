pub struct Solution;

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let k = k as usize;
        let v = unsafe { s.as_bytes_mut() };
        v.chunks_mut(2 * k).for_each(|c| {
            let n = c.len();
            c[..k.min(n)].reverse();
        });
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("bacdfeg", Solution::reverse_str("abcdefg".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!("bacd", Solution::reverse_str("abcd".to_string(), 2));
    }
}
