pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        fn dfs(x: i32, result: &mut String, max: i32, k: i32, visited: &mut HashSet<i32>) {
            for i in 0..k {
                let next = x * 10 + i;
                if !visited.contains(&next) {
                    visited.insert(next);
                    dfs(next % max, result, max, k, visited);
                    result.push((i as u8 + b'0') as char);
                }
            }
        }
        let max = 10i32.pow(n as u32 - 1);
        let mut result = String::new();
        let mut visisted = HashSet::new();
        dfs(0, &mut result, max, k, &mut visisted);
        result += &"0".repeat(n as usize - 1);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::crack_safe(1, 2);
        assert!(["01", "10"].contains(&result.as_str()));
    }

    #[test]
    fn case2() {
        let result = Solution::crack_safe(2, 2);
        assert!(["00110", "01100", "10011", "11001"].contains(&result.as_str()));
    }
}
