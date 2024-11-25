pub struct Solution;

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (b.len(), b[0].len());
        let mut result = vec![vec!['.'; m]; n];
        for (i, r) in b.iter().enumerate() {
            let mut obs = n;
            for (j, &c) in r.iter().enumerate().rev() {
                match c {
                    '#' => {
                        result[obs - 1][m - 1 - i] = '#';
                        obs -= 1;
                    }
                    '*' => {
                        result[j][m - 1 - i] = '*';
                        obs = j;
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let b = [["#", ".", "#"]]
            .iter()
            .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        let expected: Vec<Vec<char>> = [["."], ["#"], ["#"]]
            .iter()
            .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(expected, Solution::rotate_the_box(b));
    }

    #[test]
    fn case2() {
        let b = [["#", ".", "*", "."], ["#", "#", "*", "."]]
            .iter()
            .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        let expected: Vec<Vec<char>> = [["#", "."], ["#", "#"], ["*", "*"], [".", "."]]
            .iter()
            .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(expected, Solution::rotate_the_box(b));
    }

    #[test]
    fn case3() {
        let b = [
            ["#", "#", "*", ".", "*", "."],
            ["#", "#", "#", "*", ".", "."],
            ["#", "#", "#", ".", "#", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
        let expected: Vec<Vec<char>> = [
            [".", "#", "#"],
            [".", "#", "#"],
            ["#", "#", "*"],
            ["#", "*", "."],
            ["#", ".", "*"],
            ["#", ".", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(expected, Solution::rotate_the_box(b));
    }
}
