pub struct Solution;

impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() == 1 {
            return 0;
        }
        stock_prices.sort_unstable();
        let sps = stock_prices;
        let mut result = 1;
        for (i, p) in sps.iter().enumerate().skip(2) {
            let p2 = &sps[i - 2];
            let p1 = &sps[i - 1];
            if (p[1] - p1[1]) * (p1[0] - p2[0]) != (p1[1] - p2[1]) * (p[0] - p1[0]) {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_lines(lines: &[[i32; 2]]) -> Vec<Vec<i32>> {
        lines.iter().map(|l| l.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let lines = parse_lines(&[
            [1, 7],
            [2, 6],
            [3, 5],
            [4, 4],
            [5, 4],
            [6, 3],
            [7, 2],
            [8, 1],
        ]);
        assert_eq!(3, Solution::minimum_lines(lines));
    }
}
