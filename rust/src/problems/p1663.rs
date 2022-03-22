pub struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, mut k: i32) -> String {
        let mut result = String::with_capacity(n as usize);
        for i in (0..n).rev() {
            let c = (k - i * 26).max(1);
            result.push(((c as u8 - 1) + b'a') as char);
            k -= c;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("aay", Solution::get_smallest_string(3, 27));
    }

    #[test]
    fn case2() {
        assert_eq!("aaszz", Solution::get_smallest_string(5, 73));
    }
}
