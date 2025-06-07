pub struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let n = s.len();
        let mut s = s.into_bytes();
        let mut pos = (0..26).map(|_| Vec::with_capacity(n)).collect::<Vec<_>>();
        let mut mask = 0u32;
        for i in 0..n {
            let b = s[i];
            if b == b'*' {
                let j = mask.trailing_zeros() as usize;
                let k = pos[j].pop().unwrap();
                s[k] = b'*';
                if pos[j].is_empty() {
                    mask &= !(1 << j);
                }
            } else {
                let j = (b - b'a') as usize;
                pos[j].push(i);
                mask |= 1 << j;
            }
        }
        let r = s.into_iter().filter(|&b| b != b'*').collect();
        unsafe { String::from_utf8_unchecked(r) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("aab", Solution::clear_stars("aaba*".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("abc", Solution::clear_stars("abc".to_string()));
    }
}
