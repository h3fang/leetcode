pub struct Solution;

impl Solution {
    pub fn get_smallest_string(mut s: String) -> String {
        let b = unsafe { s.as_mut_vec() };
        for i in 0..b.len() - 1 {
            if b[i] > b[i + 1] && (b[i] - b'0') % 2 == (b[i + 1] - b'0') % 2 {
                b.swap(i, i + 1);
                break;
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
        assert_eq!("43520", Solution::get_smallest_string("45320".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("001", Solution::get_smallest_string("001".to_string()));
    }
}
