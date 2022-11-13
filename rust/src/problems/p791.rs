pub struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, mut s: String) -> String {
        let mut t = [0; 256];
        for (i, &b) in order.as_bytes().iter().enumerate() {
            t[b as usize] = i as u8;
        }
        let bytes = unsafe { s.as_bytes_mut() };
        bytes.sort_unstable_by_key(|&b| t[b as usize]);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let order = "cba".to_string();
        let s = "abcd".to_string();

        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        let result = result
            .chars()
            .filter(|c| order.contains(*c))
            .collect::<String>();

        assert_eq!("cba", result);
    }

    #[test]
    fn case2() {
        let order = "cbafg".to_string();
        let s = "abcd".to_string();

        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        let result = result
            .chars()
            .filter(|c| order.contains(*c))
            .collect::<String>();

        assert_eq!("cba", result);
    }
}
