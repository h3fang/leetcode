pub struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut odd = 0;
        for n in &position {
            if n % 2 == 1 {
                odd += 1;
            }
        }
        odd.min(position.len() as i32 - odd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 1000000000]));
    }
}
