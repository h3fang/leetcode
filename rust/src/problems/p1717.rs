pub struct Solution;

impl Solution {
    pub fn maximum_gain(mut s: String, mut x: i32, mut y: i32) -> i32 {
        if x < y {
            let s = unsafe { s.as_bytes_mut() };
            s.iter_mut().for_each(|c| {
                if *c == b'a' {
                    *c = b'b';
                } else if *c == b'b' {
                    *c = b'a';
                }
            });
            (x, y) = (y, x);
        }
        let mut result = 0;
        let (mut c_a, mut c_b) = (0, 0);
        for b in s.bytes() {
            match b {
                b'a' => {
                    c_a += 1;
                }
                b'b' => {
                    if c_a > 0 {
                        result += x;
                        c_a -= 1;
                    } else {
                        c_b += 1;
                    }
                }
                _ => {
                    result += y * c_a.min(c_b);
                    c_a = 0;
                    c_b = 0;
                }
            }
        }
        result + y * c_a.min(c_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(19, Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(
            20,
            Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4)
        );
    }
}
