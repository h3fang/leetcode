pub struct Solution;

impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let mut ids = (0..n).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| nums2[i]);
        let mut f = vec![0; n + 1];
        for (i, k) in ids.into_iter().enumerate() {
            let (a, b) = (nums1[k], nums2[k]);
            for j in (1..=i + 1).rev() {
                f[j] = f[j].max(f[j - 1] + a + b * j as i32);
            }
        }
        let s1: i32 = nums1.into_iter().sum();
        let s2: i32 = nums2.into_iter().sum();
        for (t, f) in f.into_iter().enumerate() {
            if s1 + s2 * t as i32 - f <= x {
                return t as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4));
    }
}
