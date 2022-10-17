pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut m = HashMap::new();
        for (right, &f) in fruits.iter().enumerate() {
            *m.entry(f).or_insert(0) += 1;
            while m.len() > 2 {
                *m.get_mut(&fruits[left]).unwrap() -= 1;
                if *m.get(&fruits[left]).unwrap() == 0 {
                    m.remove(&fruits[left]);
                }
                left += 1;
            }
            result = result.max(right - left + 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::total_fruit(vec![1, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::total_fruit(vec![0, 1, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::total_fruit(vec![1, 2, 3, 2, 2]));
    }
}
