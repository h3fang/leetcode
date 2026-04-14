pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();
        let m = robot.len();
        let mut f = vec![i64::MAX / 2; m + 1];
        f[0] = 0;
        let mut q = VecDeque::with_capacity(m + 1);
        for e in factory {
            let (position, limit) = (e[0], e[1]);
            let mut sum = 0;
            q.clear();
            q.push_back((0, 0));

            for (j, &r) in robot.iter().enumerate() {
                let j = j + 1;
                sum += (r - position).abs() as i64;

                let v = f[j] - sum;
                while let Some(e) = q.back()
                    && e.1 >= v
                {
                    q.pop_back();
                }
                q.push_back((j, v));

                while let Some(e) = q.front()
                    && e.0 + (limit as usize) < j
                {
                    q.pop_front();
                }

                f[j] = sum + q.front().unwrap().1;
            }
        }
        f[m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let robot = vec![0, 4, 6];
        let factory = [[2, 2], [6, 2]].iter().map(|f| f.to_vec()).collect();
        assert_eq!(4, Solution::minimum_total_distance(robot, factory));
    }

    #[test]
    fn case2() {
        let robot = vec![1, -1];
        let factory = [[-2, 1], [2, 1]].iter().map(|f| f.to_vec()).collect();
        assert_eq!(2, Solution::minimum_total_distance(robot, factory));
    }
}
