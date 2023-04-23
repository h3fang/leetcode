pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX / 2; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let mut w = 0;
            let mut h = 0;
            for (j, b) in books[0..=i].iter().enumerate().rev() {
                w += b[0];
                if w > shelf_width {
                    break;
                }
                h = h.max(b[1]);
                dp[i + 1] = dp[i + 1].min(dp[j] + h);
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let books = [[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(6, Solution::min_height_shelves(books, 4));
    }

    #[test]
    fn case2() {
        let books = [[1, 3], [2, 4], [3, 2]]
            .iter()
            .map(|b| b.to_vec())
            .collect();
        assert_eq!(4, Solution::min_height_shelves(books, 6));
    }
}
