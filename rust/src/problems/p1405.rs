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
    fn case1() {
        let a = 1;
        let b = 1;
        let c = 7;
        let result = Solution::longest_diverse_string(a, b, c);
        let mut na = 0;
        let mut nb = 0;
        let mut nc = 0;
        for r in result.as_bytes() {
            match r {
                b'a' => na += 1,
                b'b' => nb += 1,
                b'c' => nc += 1,
                _ => unreachable!(),
            }
        }
        assert!(na <= a && nb <= b && nc <= c);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn case2() {
        let a = 2;
        let b = 2;
        let c = 1;
        let result = Solution::longest_diverse_string(a, b, c);
        let mut na = 0;
        let mut nb = 0;
        let mut nc = 0;
        for r in result.as_bytes() {
            match r {
                b'a' => na += 1,
                b'b' => nb += 1,
                b'c' => nc += 1,
                _ => unreachable!(),
            }
        }
        assert!(na <= a && nb <= b && nc <= c);
        assert_eq!(result.len(), 5);
    }
}
