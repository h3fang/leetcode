pub struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let (cost1, cost2, cost_both, need1, need2) = (
            i64::from(cost1),
            i64::from(cost2),
            i64::from(cost_both),
            i64::from(need1),
            i64::from(need2),
        );
        let c1 = cost1 * need1 + cost2 * need2;
        let c2 = need1.min(need2) * cost_both
            + (need1 - need2).max(0) * cost1
            + (need2 - need1).max(0) * cost2;
        let c3 = need1.max(need2) * cost_both;
        c1.min(c2).min(c3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_cost(3, 2, 1, 3, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(22, Solution::minimum_cost(5, 4, 15, 2, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_cost(5, 4, 15, 0, 0));
    }
}
