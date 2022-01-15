use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut locations: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, a) in arr.iter().enumerate() {
            locations.entry(*a).or_default().push(i);
        }
        let mut visited = vec![false; n];
        let mut q = VecDeque::with_capacity(n);
        let mut jumps = 0;
        q.push_back(0);
        visited[0] = true;
        while !q.is_empty() {
            let k = q.len();
            for _ in 0..k {
                let i = q.pop_front().unwrap();
                if i == n - 1 {
                    return jumps;
                }
                if i > 0 && !visited[i - 1] {
                    visited[i - 1] = true;
                    q.push_back(i - 1);
                }
                if i + 1 < n && !visited[i + 1] {
                    visited[i + 1] = true;
                    q.push_back(i + 1);
                }
                if let Some(ids) = locations.get(&arr[i]) {
                    for &j in ids {
                        if !visited[j] {
                            visited[j] = true;
                            q.push_back(j);
                        }
                    }
                    locations.remove(&arr[i]);
                }
            }
            jumps += 1;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_jumps(vec![7]));
    }
}
