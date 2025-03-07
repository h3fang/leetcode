pub struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let white: usize = grid[i..i + 2]
                    .iter()
                    .map(|r| r[j..j + 2].iter().filter(|&&c| c == 'W').count())
                    .sum();
                if white >= 3 || white <= 1 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [["B", "W", "B"], ["B", "W", "W"], ["B", "W", "B"]]
            .iter()
            .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        assert!(Solution::can_make_square(grid));
    }

    #[test]
    fn case2() {
        let grid = [["B", "W", "B"], ["W", "B", "W"], ["B", "W", "B"]]
            .iter()
            .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        assert!(!Solution::can_make_square(grid));
    }

    #[test]
    fn case3() {
        let grid = [["B", "W", "B"], ["B", "W", "W"], ["B", "W", "W"]]
            .iter()
            .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        assert!(Solution::can_make_square(grid));
    }
}
