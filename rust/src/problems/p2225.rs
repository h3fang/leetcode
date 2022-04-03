use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = BTreeMap::new();
        for m in matches {
            let _ = winners.entry(m[0]).or_insert(0);
            *winners.entry(m[1]).or_default() += 1;
        }
        let mut win = vec![];
        let mut lost = vec![];
        for (k, v) in winners {
            if v == 0 {
                win.push(k);
            } else if v == 1 {
                lost.push(k);
            }
        }
        vec![win, lost]
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
