pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut s = s.into_bytes();
        s.iter_mut().for_each(|b| *b -= b'0');
        let n = s.len();
        let step = gcd(b, n as i32) as usize;
        let g = gcd(a, 10) as u8;

        let f = |t: &mut [u8], start: usize| {
            let delta = t[start] % g + 10 - t[start];
            for i in (start..n).step_by(2) {
                t[i] = (t[i] + delta) % 10;
            }
        };

        let mut ans = vec![u8::MAX];

        for _ in (0..n).step_by(step) {
            let mut t = s.clone();
            if step & 1 == 1 {
                f(&mut t, 0);
            }
            f(&mut t, 1);
            ans = ans.min(t);
            s.rotate_right(step);
        }

        ans.iter_mut().for_each(|e| *e += b'0');
        unsafe { String::from_utf8_unchecked(ans) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "2050",
            Solution::find_lex_smallest_string("5525".to_string(), 9, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "24",
            Solution::find_lex_smallest_string("74".to_string(), 5, 1)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "0011",
            Solution::find_lex_smallest_string("0011".to_string(), 4, 2)
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "00553311",
            Solution::find_lex_smallest_string("43987654".to_string(), 7, 3)
        );
    }
}
