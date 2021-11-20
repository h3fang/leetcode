use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *map.entry(n).or_default() += 1;
        }

        let mut result = 0;
        for (k, v) in map.iter() {
            if let Some(v_next) = map.get(&(k + 1)) {
                result = result.max(v + v_next);
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
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        assert_eq!(5, Solution::find_lhs(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(2, Solution::find_lhs(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(0, Solution::find_lhs(nums));
    }
}
