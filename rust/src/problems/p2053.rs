use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut curr = HashSet::new();
        let mut dup = HashSet::new();
        for s in &arr {
            if curr.contains(&s) {
                dup.insert(s);
            } else {
                curr.insert(s);
            }
        }
        for s in &arr {
            if !dup.contains(s) {
                k -= 1;
                if k == 0 {
                    return s.to_string();
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = ["d", "b", "c", "b", "c", "a"];
        let arr = arr.iter().map(|s| s.to_string()).collect();
        let k = 2;
        assert_eq!("a".to_string(), Solution::kth_distinct(arr, k));
    }

    #[test]
    fn case2() {
        let arr = ["aaa", "aa", "a"];
        let arr = arr.iter().map(|s| s.to_string()).collect();
        let k = 1;
        assert_eq!("aaa".to_string(), Solution::kth_distinct(arr, k));
    }

    #[test]
    fn case3() {
        let arr = ["a", "b", "a"];
        let arr = arr.iter().map(|s| s.to_string()).collect();
        let k = 3;
        assert_eq!("".to_string(), Solution::kth_distinct(arr, k));
    }
}
