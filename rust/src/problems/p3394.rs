pub struct Solution;

fn check(lines: &mut [(i32, i32)]) -> bool {
    lines.sort_unstable();
    let (mut cuts, mut sum) = (0, 0);
    for (_, end) in lines.iter().take(lines.len() - 1) {
        sum += end;
        if sum == 0 {
            cuts += 1;
        }
        if cuts == 2 {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut lines = Vec::with_capacity(rectangles.len() * 2);

        // vertical
        for r in &rectangles {
            lines.push((r[1], 1));
            lines.push((r[3], -1));
        }
        if check(&mut lines) {
            return true;
        }

        // horizontal
        lines.clear();
        for r in &rectangles {
            lines.push((r[0], 1));
            lines.push((r[2], -1));
        }
        check(&mut lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rectangles = [[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::check_valid_cuts(5, rectangles));
    }

    #[test]
    fn case2() {
        let rectangles = [[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::check_valid_cuts(4, rectangles));
    }

    #[test]
    fn case3() {
        let rectangles = [
            [0, 2, 2, 4],
            [1, 0, 3, 2],
            [2, 2, 3, 4],
            [3, 0, 4, 2],
            [3, 2, 4, 4],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert!(!Solution::check_valid_cuts(4, rectangles));
    }
}
