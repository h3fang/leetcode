pub struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut result = 0u8;
        let mut max = 0;
        let mut last = 0;
        let keys = keys_pressed.as_bytes();
        for i in 0..keys.len() {
            let t = release_times[i] - last;
            last = release_times[i];
            if (t == max && keys[i] > result) || t > max {
                max = t;
                result = keys[i];
            }
        }
        result as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            'c',
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            'a',
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string())
        );
    }
}
