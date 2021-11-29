use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut prefix = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut result = 0;
        for (i, &n) in nums.iter().enumerate() {
            prefix += if n == 1 { 1 } else { -1 };
            if let Some(&j) = map.get(&prefix) {
                result = result.max(i as i32 - j);
            } else {
                map.insert(prefix, i as i32);
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
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
    }
}
