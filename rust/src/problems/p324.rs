pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut arr = nums.clone();
        arr.sort_unstable();
        let n = nums.len();
        let x = n.div_ceil(2);
        let mut j = x as i32 - 1;
        let mut k = n as i32 - 1;
        for i in (0..n).step_by(2) {
            nums[i] = arr[j as usize];
            if i + 1 < n {
                nums[i + 1] = arr[k as usize];
            }
            j -= 1;
            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(nums: &[i32]) -> bool {
        nums.windows(2)
            .enumerate()
            .all(|(i, w)| if i % 2 == 0 { w[0] < w[1] } else { w[0] > w[1] })
    }

    #[test]
    fn case1() {
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut nums);
        assert!(is_valid(&nums));
    }
}
