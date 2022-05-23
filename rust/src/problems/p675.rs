use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        fn shortest_path(forest: &[Vec<i32>], curr: (i32, i32), target: (i32, i32)) -> i32 {
            if curr == target {
                return 0;
            }

            let m = forest.len() as i32;
            let n = forest[0].len() as i32;

            let mut visited = vec![false; (m * n) as usize];
            let mut q = VecDeque::new();
            q.push_back(curr);
            let mut d = 0;
            while !q.is_empty() {
                let k = q.len();
                for _ in 0..k {
                    let (i, j) = q.pop_front().unwrap();
                    if (i, j) == target {
                        return d;
                    }
                    visited[(i * n + j) as usize] = true;
                    for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                        if i1 < 0
                            || j1 < 0
                            || i1 >= m
                            || j1 >= n
                            || forest[i1 as usize][j1 as usize] == 0
                            || visited[(i1 * n + j1) as usize]
                        {
                            continue;
                        }
                        visited[(i1 * n + j1) as usize] = true;
                        q.push_back((i1, j1));
                    }
                }
                d += 1;
            }
            -1
        }

        let mut trees = vec![];
        for i in 0..forest.len() {
            for j in 0..forest[0].len() {
                if forest[i][j] > 1 {
                    trees.push((forest[i][j], (i as i32, j as i32)));
                }
            }
        }
        trees.sort_unstable();
        let mut result = 0;
        let mut curr = (0, 0);
        for (_, target) in trees {
            let d = shortest_path(&forest, curr, target);
            if d == -1 {
                return -1;
            }
            result += d;
            curr = target;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let forest = [[1, 2, 3], [0, 0, 4], [7, 6, 5]];
        let forest = forest.iter().map(|r| r.to_vec()).collect();
        assert_eq!(6, Solution::cut_off_tree(forest));
    }

    #[test]
    fn case2() {
        let forest = [[1, 2, 3], [0, 0, 0], [7, 6, 5]];
        let forest = forest.iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::cut_off_tree(forest));
    }
}
