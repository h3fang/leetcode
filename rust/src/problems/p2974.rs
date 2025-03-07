pub struct Solution;

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        for c in nums.chunks_mut(2) {
            c.swap(0, 1);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3, 2, 5, 4], Solution::number_game(vec![5, 4, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![5, 2], Solution::number_game(vec![2, 5]));
    }
}
