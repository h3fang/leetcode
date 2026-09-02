pub struct Solution;

impl Solution {
    pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1]);
        let mut s = Vec::with_capacity(tasks.len());
        s.push((-1, -1, 0));
        for t in tasks {
            let i = s.partition_point(|e| e.0 < t[0]);
            let (_, r, d) = s[i - 1];
            let mut duration = t[2] - (s.last().unwrap().2 - d);
            if t[0] <= r {
                duration -= r - t[0] + 1;
            }

            if duration <= 0 {
                continue;
            }

            while duration >= t[1] - s.last().unwrap().1 {
                let (l, r, _) = s.pop().unwrap();
                duration += r - l + 1;
            }
            s.push((t[1] - duration + 1, t[1], s.last().unwrap().2 + duration));
        }
        s.last().unwrap().2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tasks = [[2, 3, 1], [4, 5, 1], [1, 5, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(2, Solution::find_minimum_time(tasks));
    }

    #[test]
    fn case2() {
        let tasks = [[1, 3, 2], [2, 5, 3], [5, 6, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(4, Solution::find_minimum_time(tasks));
    }
}
