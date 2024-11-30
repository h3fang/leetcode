pub struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (mut one, mut two) = (0, 0);
        for x in nums {
            if x < 10 {
                one += x;
            } else {
                two += x;
            }
        }
        one != two
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::can_alice_win(vec![1, 2, 3, 4, 10]));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_alice_win(vec![5, 5, 5, 25]));
    }
}
