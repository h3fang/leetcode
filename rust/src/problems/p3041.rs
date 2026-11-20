pub struct Solution;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let (mut f0, mut f1, mut f2) = (0, 0, 0);
        let (mut ans, mut pre) = (0, 0);

        for x in nums {
            if x == pre + 1 {
                (f0, f1) = (f1, f2);
            } else if x == pre + 2 {
                (f0, f1) = (f2, 0);
            } else if x > pre + 2 {
                (f0, f1) = (0, 0);
            };
            f2 = f1 + 1;
            f1 = f0 + 1;
            ans = ans.max(f1).max(f2);
            pre = x;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_selected_elements(vec![2, 1, 5, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_selected_elements(vec![1, 4, 7, 10]));
    }
}
