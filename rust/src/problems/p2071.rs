use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        workers.sort_unstable();
        tasks.sort_unstable();
        let n = tasks.len().min(workers.len());

        fn can_be_done(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32) -> bool {
            let mut ws: BTreeMap<i32, i32> = BTreeMap::new();
            for w in workers {
                *ws.entry(*w).or_default() += 1;
            }
            for t in tasks.iter().rev() {
                let (&w, c) = ws.iter_mut().last().unwrap();
                if w >= *t {
                    if *c == 1 {
                        ws.remove(&w);
                    } else {
                        *c -= 1;
                    }
                } else if ws.keys().next().unwrap() < t {
                    if pills > 0 {
                        if let Some((&w, c)) = ws.range_mut(*t - strength..).next() {
                            if *c == 1 {
                                ws.remove(&w);
                            } else {
                                *c -= 1;
                            }
                            pills -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        }

        let mut left = 0;
        let mut right = n;
        while left <= right {
            let mid = (right + left) / 2;
            if can_be_done(&tasks[..mid], &workers[n - mid..], pills, strength) {
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

    // 50% faster than above method (using BTreeMap) on US server, but TLE on CN server
    // may be releated to rust version (US: 1.45.2, CN: 1.54.0) or server specs
    pub fn max_task_assign_binary_on_vec(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        workers.sort_unstable();
        tasks.sort_unstable();
        let n = tasks.len().min(workers.len());

        fn binary_search(workers: &[i32], t: i32) -> Option<usize> {
            let n = workers.len();
            let mut left = 0;
            let mut right = n - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                match workers[mid].cmp(&t) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return Some(mid),
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                }
            }
            if left == n {
                None
            } else {
                Some(left)
            }
        }

        fn can_be_done(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32) -> bool {
            let mut workers = workers.to_owned();
            for t in tasks.iter().rev() {
                if workers.last().unwrap() >= t {
                    workers.pop();
                } else if workers.first().unwrap() < t {
                    if pills > 0 {
                        let p = binary_search(&workers, *t - strength);
                        if let Some(p) = p {
                            workers.remove(p);
                            pills -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        }

        let mut left = 0;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            if can_be_done(&tasks[..mid], &workers[n - mid..], pills, strength) {
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
            Solution::max_task_assign_binary_on_vec(
                tasks.clone(),
                workers.clone(),
                pills,
                strength
            )
        );
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
            Solution::max_task_assign_binary_on_vec(
                tasks.clone(),
                workers.clone(),
                pills,
                strength
            )
        );
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
            Solution::max_task_assign_binary_on_vec(
                tasks.clone(),
                workers.clone(),
                pills,
                strength
            )
        );
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
            Solution::max_task_assign_binary_on_vec(
                tasks.clone(),
                workers.clone(),
                pills,
                strength
            )
        );
        assert_eq!(
            3,
            Solution::max_task_assign(tasks, workers, pills, strength)
        );
    }
}
