pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut s = vec![];
        let mut top = 0;
        for b in num.as_bytes() {
            let b = b - b'0';
            while top > 0 && s[top - 1] > b && k > 0 {
                s.pop();
                top -= 1;
                k -= 1;
            }
            s.push(b);
            top += 1;
        }
        let r = s
            .iter()
            .take(s.len() - k as usize)
            .map(|b| (b + b'0') as char)
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();
        if r.is_empty() { "0".to_string() } else { r }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("1219", Solution::remove_kdigits("1432219".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert_eq!("200", Solution::remove_kdigits("10200".to_string(), 1));
    }
}
