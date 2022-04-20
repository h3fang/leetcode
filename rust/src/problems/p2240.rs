pub struct Solution;

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let pen = total / cost1;
        let mut result = 0;
        for p in 0..=pen {
            result += ((total - p * cost1) / cost2 + 1) as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::ways_to_buy_pens_pencils(20, 10, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::ways_to_buy_pens_pencils(5, 10, 10));
    }
}
