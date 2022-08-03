pub struct Solution;

impl Solution {
    pub fn orderly_queue(mut s: String, k: i32) -> String {
        if k == 1 {
            let ss = s.repeat(2);
            let mut min = s.as_str();
            let n = s.len();
            for i in 1..n {
                min = min.min(&ss[i..i + n]);
            }
            min.to_string()
        } else {
            unsafe {
                let s = s.as_bytes_mut();
                s.sort_unstable();
            }
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("acb", Solution::orderly_queue("cba".into(), 1));
    }

    #[test]
    fn case2() {
        assert_eq!("aaabc", Solution::orderly_queue("baaca".into(), 3));
    }
}
