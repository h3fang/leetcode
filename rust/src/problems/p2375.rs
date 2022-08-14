pub struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let p = pattern.as_bytes();
        let mut result = vec![b'1'; p.len() + 1];
        for (i, &b) in p.iter().enumerate() {
            if b == b'I' {
                let mut used = [false; 10];
                for c in &result[..=i] {
                    used[(c - b'0') as usize] = true;
                }
                let mut min = 9;
                for (i, &u) in used.iter().enumerate().skip(1) {
                    if !u {
                        min = i as i32;
                        break;
                    }
                }
                result[i + 1] = min as u8 + b'0';
            } else {
                result[i + 1] = result[i];
                let mut j = i;
                while p[j] == b'D' {
                    result[j] += 1;
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
            }
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("123549876", Solution::smallest_number("IIIDIDDD".into()));
    }

    #[test]
    fn case2() {
        assert_eq!("4321", Solution::smallest_number("DDD".into()));
    }
}
