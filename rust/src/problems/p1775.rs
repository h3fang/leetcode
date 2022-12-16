pub struct Solution;

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn frequency_sum(nums: &[i32]) -> ([i32; 7], i32) {
            let mut f = [0; 7];
            let mut sum = 0;
            nums.iter().for_each(|&n| {
                f[n as usize] += 1;
                sum += n;
            });
            (f, sum)
        }

        fn check(f1: &[i32], f2: &[i32], mut diff: i32) -> i32 {
            let mut result = 0;
            let mut h = [0; 7];
            for (i, (n1, n2)) in f1.iter().zip(f2).enumerate().skip(1) {
                h[6 - i] += n1;
                h[i - 1] += n2;
            }
            for i in (0..6).rev() {
                if diff <= 0 {
                    break;
                }
                let t = h[i as usize].min((diff + i - 1) / i);
                result += t;
                diff -= t * i;
            }
            result
        }

        let m = nums1.len();
        let n = nums2.len();
        if 6 * m < n || 6 * n < m {
            return -1;
        }
        let (f1, s1) = frequency_sum(&nums1);
        let (f2, s2) = frequency_sum(&nums2);
        let diff = s1 - s2;
        match diff.cmp(&0) {
            std::cmp::Ordering::Less => check(&f1, &f2, -diff),
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => check(&f2, &f1, diff),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::min_operations(vec![6, 6], vec![1]));
    }
}
