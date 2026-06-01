pub struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by_key(|&x| -x);
        cost.chunks(3).map(|c| c[0] + c.get(1).unwrap_or(&0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_cost(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(23, Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(10, Solution::minimum_cost(vec![5, 5]));
    }
}
