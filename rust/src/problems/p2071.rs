pub struct Solution;

use std::collections::VecDeque;

fn can_be_done(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32) -> bool {
    let (mut i, k) = (0, tasks.len());
    let mut q = VecDeque::with_capacity(k);
    for &w in workers {
        while i < k && tasks[i] <= w + strength {
            q.push_back(tasks[i]);
            i += 1;
        }
        if q.is_empty() {
            return false;
        } else if w >= *q.front().unwrap() {
            q.pop_front();
        } else if pills > 0 {
            q.pop_back();
            pills -= 1;
        } else {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        workers.sort_unstable();
        tasks.sort_unstable();
        let (n, m) = (tasks.len(), workers.len());

        let mut left = 0;
        let mut right = n.min(m);
        while left <= right {
            let mid = (right + left) / 2;
            if can_be_done(&tasks[..mid], &workers[m - mid..], pills, strength) {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return 0;
                }
                right = mid - 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tasks = vec![3, 2, 1];
        let workers = vec![0, 3, 3];
        let pills = 1;
        let strength = 1;

        assert_eq!(
            3,
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }

    #[test]
    fn case2() {
        let tasks = vec![5, 4];
        let workers = vec![0, 0, 0];
        let pills = 1;
        let strength = 5;

        assert_eq!(
            1,
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }

    #[test]
    fn case3() {
        let tasks = vec![10, 15, 30];
        let workers = vec![0, 10, 10, 10, 10];
        let pills = 3;
        let strength = 10;

        assert_eq!(
            2,
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }

    #[test]
    fn case4() {
        let tasks = vec![5, 9, 8, 5, 9];
        let workers = vec![1, 6, 4, 2, 6];
        let pills = 1;
        let strength = 5;

        assert_eq!(
            3,
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }
}
