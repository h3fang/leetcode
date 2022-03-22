pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result = vec![];
        let mut last = [0; 26];
        let s = s.as_bytes();
        for (i, c) in s.iter().enumerate() {
            let idx = (c - b'a') as usize;
            last[idx] = i;
        }
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.iter().enumerate() {
            let idx = (c - b'a') as usize;
            end = end.max(last[idx]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = end + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![9, 7, 8],
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![10],
            Solution::partition_labels("eccbbbbdec".to_string())
        );
    }
}
