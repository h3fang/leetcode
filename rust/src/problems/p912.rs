pub struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn dc(nums: &mut [i32]) {
            let n = nums.len();
            let mut arr = nums.to_vec();
            if n <= 1 {
                return;
            }
            let m = n / 2;
            dc(&mut arr[..m]);
            dc(&mut arr[m..]);
            let mut i = 0;
            let mut j = m;
            let mut k = 0;
            while i < m && j < n {
                if arr[i] <= arr[j] {
                    nums[k] = arr[i];
                    i += 1;
                } else {
                    nums[k] = arr[j];
                    j += 1;
                }
                k += 1;
            }
            if i < m {
                for &a in &arr[i..m] {
                    nums[k] = a;
                    k += 1;
                }
            } else if j < n {
                for &a in &arr[j..n] {
                    nums[k] = a;
                    k += 1;
                }
            }
        }
        dc(&mut nums);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 2, 3, 5], Solution::sort_array(vec![5, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 0, 1, 1, 2, 5],
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0])
        );
    }
}
