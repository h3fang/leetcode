pub struct Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut f = vec![0; 201];
        for &n in &nums {
            f[(n + 100) as usize] += 1;
        }
        nums.sort_unstable_by_key(|&n| (f[(n + 100) as usize], -n));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 1, 1, 2, 2, 2],
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );
    }
}
