pub struct Solution;

fn length(mut x: i32) -> i32 {
    let mut ans = i32::from(x <= 0);
    while x != 0 {
        ans += 1;
        x /= 10;
    }
    ans
}

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len())
            .map(|j| {
                let (mut min, mut max) = (grid[0][j], grid[0][j]);
                for row in &grid {
                    let x = row[j];
                    min = min.min(x);
                    max = max.max(x);
                }
                length(min).max(length(max))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1], [22], [333]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![3], Solution::find_column_width(grid));
    }

    #[test]
    fn case2() {
        let grid = [[-15, 1, 3], [15, 7, 12], [5, 6, -2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![3, 1, 2], Solution::find_column_width(grid));
    }
}
