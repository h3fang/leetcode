pub struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut f = vec![[0; 11]; n as usize];
        for p in pick {
            f[p[0] as usize][p[1] as usize] += 1;
        }
        f.iter()
            .enumerate()
            .filter(|&(i, f)| f.iter().any(|&e| e > i as i32))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let pick = [[0, 0], [1, 0], [1, 0], [2, 1], [2, 1], [2, 0]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(2, Solution::winning_player_count(4, pick));
    }

    #[test]
    fn case2() {
        let pick = [[1, 1], [1, 2], [1, 3], [1, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(0, Solution::winning_player_count(5, pick));
    }

    #[test]
    fn case3() {
        let pick = [[1, 1], [2, 4], [2, 4], [2, 4]]
            .iter()
            .map(|p| p.to_vec())
            .collect();
        assert_eq!(1, Solution::winning_player_count(5, pick));
    }
}
