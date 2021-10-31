use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut result = 0;
        let mut q = VecDeque::new();
        let mut visited = [false; 1001];
        q.push_back(start);
        while !q.is_empty() {
            let n = q.len();
            for _ in 0..n {
                let a = q.pop_front().unwrap();
                if a == goal {
                    return result;
                }
                if !(0..=1000).contains(&a) || visited[a as usize] {
                    continue;
                }

                visited[a as usize] = true;

                for e in &nums {
                    q.push_back(a + e);
                    q.push_back(a - e);
                    q.push_back(a ^ e);
                }
            }

            result += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3];
        let start = 6;
        let goal = 4;
        assert_eq!(2, Solution::minimum_operations(nums, start, goal));
    }

    #[test]
    fn case2() {
        let nums = vec![2, 4, 12];
        let start = 2;
        let goal = 12;
        assert_eq!(2, Solution::minimum_operations(nums, start, goal));
    }

    #[test]
    fn case3() {
        let nums = vec![3, 5, 7];
        let start = 0;
        let goal = -4;
        assert_eq!(2, Solution::minimum_operations(nums, start, goal));
    }

    #[test]
    fn case4() {
        let nums = vec![2, 8, 16];
        let start = 0;
        let goal = 1;
        assert_eq!(-1, Solution::minimum_operations(nums, start, goal));
    }

    #[test]
    fn case5() {
        let nums = vec![1];
        let start = 0;
        let goal = 3;
        assert_eq!(3, Solution::minimum_operations(nums, start, goal));
    }
}
