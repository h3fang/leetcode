pub struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let (mut ans, mut prev) = (0, 0);
        for w in nums.windows(2) {
            if w[0] == w[1] {
                continue;
            }
            let curr = if w[0] > w[1] { 1 } else { -1 };
            if curr == -prev {
                ans += 1;
            }
            prev = curr;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]));
    }
}
