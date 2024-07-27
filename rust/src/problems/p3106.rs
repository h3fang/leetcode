pub struct Solution;

impl Solution {
    pub fn get_smallest_string(mut s: String, mut k: i32) -> String {
        let r = unsafe { s.as_bytes_mut() };
        for x in r {
            if k <= 0 {
                break;
            }
            if *x == b'a' {
                continue;
            }
            let min = (*x - b'a').min(b'z' - *x + 1) as i32;
            if k >= min {
                k -= min;
                *x = b'a';
            } else {
                *x -= k as u8;
                k = 0;
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
        assert_eq!("aaaz", Solution::get_smallest_string("zbbz".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "aawcd",
            Solution::get_smallest_string("xaxcd".to_string(), 4)
        );
    }

    #[test]
    fn case3() {
        assert_eq!("lol", Solution::get_smallest_string("lol".to_string(), 0));
    }
}
