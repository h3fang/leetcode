pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        while nums.len() > 1 && !nums.is_sorted() {
            let (i, sum) = nums
                .windows(2)
                .enumerate()
                .map(|(i, w)| (i, w[0] + w[1]))
                .min_by_key(|e| e.1)
                .unwrap();
            nums[i] = sum;
            nums.remove(i + 1);
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_pair_removal(vec![5, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_pair_removal(vec![1, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            9,
            Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1])
        );
    }
}
