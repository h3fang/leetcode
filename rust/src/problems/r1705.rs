pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut m = HashMap::from([(0, -1)]);
        let (mut sum, mut max, mut start) = (0, 0, -1);
        for (i, s) in array.iter().enumerate() {
            sum += if s.as_bytes()[0].is_ascii_digit() {
                1
            } else {
                -1
            };
            if let Some(j) = m.get(&sum) {
                let c = i as i32 - j;
                if c > max {
                    max = c;
                    start = j + 1;
                }
            } else {
                m.insert(sum, i as i32);
            }
        }
        array
            .into_iter()
            .skip(start as usize)
            .take(max as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let array = [
            "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I", "J",
            "K", "L", "M",
        ]
        .iter()
        .map(|a| a.to_string())
        .collect();
        let expected = [
            "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_longest_subarray(array));
    }

    #[test]
    fn case2() {
        let array = ["A", "A"].iter().map(|a| a.to_string()).collect();
        let expected = vec![String::new(); 0];
        assert_eq!(expected, Solution::find_longest_subarray(array));
    }
}
