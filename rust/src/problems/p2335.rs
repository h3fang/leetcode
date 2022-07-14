pub struct Solution;

impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort_unstable();
        if amount[2] == 0 {
            return 0;
        }
        if amount[1] > 0 {
            amount[1] -= 1;
            amount[2] -= 1;
            1 + Self::fill_cups(amount)
        } else {
            amount[2] -= 1;
            1 + Self::fill_cups(amount)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::fill_cups(vec![1, 4, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::fill_cups(vec![5, 4, 4]));
    }
}
