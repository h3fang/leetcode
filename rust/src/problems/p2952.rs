pub struct Solution;

impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_unstable();
        let (mut result, mut i, mut x, n) = (0, 0, 1, coins.len());
        while x <= target {
            if i < n && coins[i] <= x {
                x += coins[i];
                i += 1;
            } else {
                result += 1;
                x *= 2;
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
        assert_eq!(2, Solution::minimum_added_coins(vec![1, 4, 10], 9));
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::minimum_added_coins(vec![1, 1, 1], 20));
    }
}
