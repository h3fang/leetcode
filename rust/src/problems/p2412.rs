pub struct Solution;

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut max_neg_cashback = 0;
        let mut max_pos_cost = 0;
        let mut cost = 0;
        for t in transactions {
            if t[0] > t[1] {
                cost += (t[0] - t[1]) as i64;
                max_neg_cashback = max_neg_cashback.max(t[1] as i64);
            } else {
                max_pos_cost = max_pos_cost.max(t[0] as i64);
            }
        }
        cost + max_neg_cashback.max(max_pos_cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let transactions = [[2, 1], [5, 0], [4, 2]]
            .iter()
            .map(|t| t.to_vec())
            .collect();
        assert_eq!(10, Solution::minimum_money(transactions));
    }

    #[test]
    fn case2() {
        let transactions = [[3, 0], [0, 3]].iter().map(|t| t.to_vec()).collect();
        assert_eq!(3, Solution::minimum_money(transactions));
    }
}
