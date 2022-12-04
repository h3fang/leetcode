pub struct Solution;

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut result = *base_costs.iter().min().unwrap();
        fn bt(curr: i32, i: usize, target: i32, topping_costs: &[i32], result: &mut i32) {
            if (*result - target).abs() < curr - target {
                return;
            }
            if i == topping_costs.len() {
                match (curr - target).abs().cmp(&(*result - target).abs()) {
                    std::cmp::Ordering::Less => *result = curr,
                    std::cmp::Ordering::Equal => *result = (*result).min(curr),
                    std::cmp::Ordering::Greater => {}
                }
                return;
            }
            for k in 0..3 {
                let c = curr + k * topping_costs[i];
                bt(c, i + 1, target, topping_costs, result);
            }
        }
        base_costs
            .into_iter()
            .for_each(|b| bt(b, 0, target, &topping_costs, &mut result));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::closest_cost(vec![1, 7], vec![3, 4], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(17, Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::closest_cost(vec![3, 10], vec![2, 5], 9));
    }

    #[test]
    fn case4() {
        assert_eq!(10, Solution::closest_cost(vec![10], vec![1], 1));
    }
}
