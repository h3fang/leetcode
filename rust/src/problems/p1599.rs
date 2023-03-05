pub struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if 4 * boarding_cost <= running_cost {
            return -1;
        }
        let mut result = -1;
        let mut max = 0;
        let mut profit = 0;
        let mut c = 0;
        let mut customers = customers.into_iter();
        for i in 1.. {
            if let Some(e) = customers.next() {
                c += e;
            } else if c == 0 {
                break;
            }
            let q = c.min(4);
            c -= q;
            profit += q * boarding_cost - running_cost;
            if max < profit {
                max = profit;
                result = i;
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
        assert_eq!(3, Solution::min_operations_max_profit(vec![8, 3], 5, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92)
        );
    }
}
