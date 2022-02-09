pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut s = Vec::with_capacity((a + b + c) as usize);
        let mut chars = vec![(a, b'a'), (b, b'b'), (c, b'c')];

        loop {
            chars.sort_unstable();
            let mut done = true;
            for (n, c) in chars.iter_mut().rev() {
                if *n <= 0 {
                    break;
                }
                let k = s.len();
                if k >= 2 && s[k - 2] == *c && s[k - 1] == *c {
                    continue;
                }
                done = false;
                s.push(*c);
                *n -= 1;
                break;
            }
            if done {
                break;
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {}
}
