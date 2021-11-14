pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;
        for (i, n) in tickets.iter().enumerate() {
            if i <= k {
                result += tickets[k].min(*n);
            } else {
                result += (tickets[k] - 1).min(*n);
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
        assert_eq!(6, Solution::time_required_to_buy(vec![2, 3, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::time_required_to_buy(vec![5, 1, 1, 1], 0));
    }
}
