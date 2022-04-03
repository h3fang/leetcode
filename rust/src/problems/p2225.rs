use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = HashSet::new();
        let mut lost_once = HashSet::new();
        let mut lost_ntimes = HashSet::new();
        for m in matches {
            if !lost_once.contains(&m[0]) && !lost_ntimes.contains(&m[0]) {
                winners.insert(m[0]);
            }
            if !lost_ntimes.contains(&m[1]) {
                if lost_once.contains(&m[1]) {
                    lost_once.remove(&m[1]);
                    lost_ntimes.insert(m[1]);
                } else {
                    winners.remove(&m[1]);
                    lost_once.insert(m[1]);
                }
            }
        }
        let mut winners = winners.into_iter().collect::<Vec<_>>();
        winners.sort_unstable();
        let mut lost_once = lost_once.into_iter().collect::<Vec<_>>();
        lost_once.sort_unstable();
        vec![winners, lost_once]
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
