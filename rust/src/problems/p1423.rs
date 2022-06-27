pub struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let m = n - k;
        let mut sub = card_points.iter().take(m).sum::<i32>();
        let mut min = sub;
        let sum = card_points.iter().sum::<i32>();
        for i in 1..card_points.len() + 1 - m {
            sub -= card_points[i - 1];
            sub += card_points[i + m - 1];
            min = min.min(sub);
        }
        sum - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_score(vec![2, 2, 2], 2));
    }
}
