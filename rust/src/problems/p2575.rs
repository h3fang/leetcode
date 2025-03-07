pub struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut r = 0;
        word.as_bytes()
            .iter()
            .map(|&b| {
                r = (r * 10 + (b - b'0') as i64) % m as i64;
                if r == 0 {
                    1
                } else {
                    0
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0],
            Solution::divisibility_array("998244353".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 1, 0, 1],
            Solution::divisibility_array("1010".to_string(), 10)
        );
    }
}
