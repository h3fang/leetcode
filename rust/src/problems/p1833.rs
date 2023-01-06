pub struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        costs
            .into_iter()
            .take_while(|c| {
                coins -= c;
                coins >= 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20));
    }
}
