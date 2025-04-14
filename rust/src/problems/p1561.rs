pub struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let n = piles.len() / 3;
        piles.sort_unstable();
        piles[n..].chunks(2).map(|c| c[0]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::max_coins(vec![2, 4, 1, 2, 7, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_coins(vec![2, 4, 5]));
    }

    #[test]
    fn case3() {
        assert_eq!(18, Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
    }
}
