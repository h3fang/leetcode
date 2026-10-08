pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let (mut sum, mut ans) = (0i64, -1);
        for x in nums {
            sum += x as i64;
            if sum > 2 * x as i64 {
                ans = sum;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::largest_perimeter(vec![5, 5, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::largest_perimeter(vec![5, 5, 50]));
    }
}
