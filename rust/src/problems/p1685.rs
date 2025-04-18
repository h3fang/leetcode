pub struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut pre, mut suf) = (0, nums.iter().sum::<i32>());
        nums.into_iter()
            .enumerate()
            .map(|(i, x)| {
                let y = i as i32 * x - pre + suf - (n - i) as i32 * x;
                pre += x;
                suf -= x;
                y
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![4, 3, 5],
            Solution::get_sum_absolute_differences(vec![2, 3, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![24, 15, 13, 15, 21],
            Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10])
        );
    }
}
