pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lost = HashMap::new();
        for m in matches {
            lost.entry(m[0]).or_insert(0);
            *lost.entry(m[1]).or_default() += 1;
        }
        let mut win = Vec::with_capacity(lost.len());
        let mut lost_once = Vec::with_capacity(lost.len());
        for (k, v) in lost {
            if v == 0 {
                win.push(k);
            } else if v == 1 {
                lost_once.push(k);
            }
        }
        win.sort_unstable();
        lost_once.sort_unstable();
        vec![win, lost_once]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matches = [
            [1, 3],
            [2, 3],
            [3, 6],
            [5, 6],
            [5, 7],
            [4, 5],
            [4, 8],
            [4, 9],
            [10, 4],
            [10, 9],
        ];
        let matches = matches.iter().map(|m| m.to_vec()).collect();
        assert_eq!(
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]],
            Solution::find_winners(matches)
        );
    }

    #[test]
    fn case2() {
        let matches = [[2, 3], [1, 3], [5, 4], [6, 4]];
        let matches = matches.iter().map(|m| m.to_vec()).collect();
        assert_eq!(
            vec![vec![1, 2, 5, 6], vec![]],
            Solution::find_winners(matches)
        );
    }
}
