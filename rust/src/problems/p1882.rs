pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut idle = BinaryHeap::with_capacity(servers.len());
        let mut busy: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::with_capacity(servers.len());

        for (id, &weight) in servers.iter().enumerate() {
            idle.push(Reverse((weight, id as i32)));
        }

        let mut ans = Vec::with_capacity(tasks.len());
        let mut time = 0;

        for (i, task) in tasks.into_iter().enumerate() {
            time = time.max(i as i64);
            while busy.peek().is_some_and(|&Reverse((t, _))| t <= time) {
                let Reverse((_, id)) = busy.pop().unwrap();
                idle.push(Reverse((servers[id as usize], id)));
            }

            if idle.is_empty() {
                time = busy.peek().unwrap().0.0;

                while busy.peek().is_some_and(|&Reverse((t, _))| t <= time) {
                    let Reverse((_, id)) = busy.pop().unwrap();
                    idle.push(Reverse((servers[id as usize], id)));
                }
            }

            let Reverse((_, id)) = idle.pop().unwrap();
            ans.push(id);
            busy.push(Reverse((time + task as i64, id)));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, 2, 0, 2, 1, 2],
            Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 4, 1, 4, 1, 3, 2],
            Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1])
        );
    }
}
