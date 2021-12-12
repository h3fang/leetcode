use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        let mut g = vec![vec![]; n];
        for i in 0..n - 1 {
            let a = &bombs[i];
            for j in i + 1..n {
                let b = &bombs[j];
                let dx = (a[0] - b[0]) as i64;
                let dy = (a[1] - b[1]) as i64;
                let d = dx * dx + dy * dy;
                let r1 = a[2] as i64;
                let r2 = b[2] as i64;
                if d <= r1 * r1 {
                    g[i].push(j);
                }
                if d <= r2 * r2 {
                    g[j].push(i);
                }
            }
        }
        let mut result = 0;
        for i in 0..n {
            let mut visited = vec![false; n];
            let mut q = VecDeque::new();
            let mut count = 0;
            q.push_back(i);
            visited[i] = true;

            while let Some(c) = q.pop_front() {
                count += 1;
                for &child in &g[c] {
                    if !visited[child] {
                        visited[child] = true;
                        q.push_back(child);
                    }
                }
            }

            result = result.max(count);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bombs = vec![vec![1, 1, 100000], vec![100000, 100000, 1]];
        assert_eq!(1, Solution::maximum_detonation(bombs));
    }

    #[test]
    fn case2() {
        let bombs = [
            [54, 95, 4],
            [99, 46, 3],
            [29, 21, 3],
            [96, 72, 8],
            [49, 43, 3],
            [11, 20, 3],
            [2, 57, 1],
            [69, 51, 7],
            [97, 1, 10],
            [85, 45, 2],
            [38, 47, 1],
            [83, 75, 3],
            [65, 59, 3],
            [33, 4, 1],
            [32, 10, 2],
            [20, 97, 8],
            [35, 37, 3],
        ];
        let bombs = bombs.iter().map(|b| b.to_vec()).collect();
        assert_eq!(1, Solution::maximum_detonation(bombs));
    }

    #[test]
    fn case3() {
        let bombs = [[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]];
        let bombs = bombs.iter().map(|b| b.to_vec()).collect();
        assert_eq!(5, Solution::maximum_detonation(bombs));
    }
}
