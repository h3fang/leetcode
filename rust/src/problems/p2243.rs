pub struct Solution;

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        while s.len() > k {
            s = s
                .as_bytes()
                .chunks(k)
                .map(|g| g.iter().map(|d| (d - b'0') as i32).sum::<i32>().to_string())
                .collect();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("135", Solution::digit_sum("11111222223".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert_eq!("000", Solution::digit_sum("00000000".to_string(), 3));
    }
}
