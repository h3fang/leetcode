pub struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut in_deg = vec![0; n as usize];
        for e in edges {
            in_deg[e[1] as usize] += 1;
        }
        let i = in_deg.iter().position(|&d| d == 0).unwrap();
        match in_deg[i + 1..].iter().position(|&d| d == 0) {
            Some(_) => -1,
            None => i as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(0, Solution::find_champion(3, edges));
    }

    #[test]
    fn case2() {
        let edges = [[0, 2], [1, 3], [1, 2]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(-1, Solution::find_champion(4, edges));
    }
}
