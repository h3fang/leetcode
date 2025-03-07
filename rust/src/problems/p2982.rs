pub struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut freqs = vec![vec![]; 26];
        let (mut last, mut c) = (b' ', 0);
        for &b in s.as_bytes().iter().chain(b" ") {
            if b != last {
                if c > 0 {
                    freqs[(last - b'a') as usize].push(c);
                }
                last = b;
                c = 1;
            } else {
                c += 1;
            }
        }
        let r = freqs
            .iter_mut()
            .map(|f| {
                f.sort_unstable_by_key(|e| -e);
                f.resize(3, i32::MIN / 2);
                if f[0] >= f[1] + 2 {
                    f[0] - 2
                } else if f[0] == f[1] + 1 {
                    f[0] - 1
                } else {
                    f[2].max(f[0] - 1)
                }
            })
            .max()
            .unwrap();
        if r == 0 {
            -1
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::maximum_length("aaaa".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::maximum_length("abcdef".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::maximum_length("abcaba".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::maximum_length("acc".to_string()));
    }

    #[test]
    fn case5() {
        assert_eq!(
            11,
            Solution::maximum_length("ceeeeeeeeeeeebmmmfffeeeeeeeeeeeewww".to_string())
        );
    }
}
