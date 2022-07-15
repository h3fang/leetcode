pub struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut d = Vec::with_capacity(nums1.len() + 1);
        let mut result = 0;
        for (a, b) in nums1.iter().zip(&nums2) {
            let diff = (a - b).abs() as i64;
            if diff > 0 {
                d.push(diff);
                result += diff * diff;
            }
        }
        d.sort_unstable_by(|a, b| b.cmp(a));
        d.push(0);
        let mut k = (k1 + k2) as i64;
        for (i, w) in d.windows(2).enumerate() {
            let i = i as i64;
            let c = (w[0] - w[1]) * (i + 1);
            result -= w[0] * w[0];
            if c < k {
                k -= c;
                continue;
            }
            let v = w[0] - k / (i + 1);
            let a = k % (i + 1);
            return result + a * (v - 1) * (v - 1) + (i + 1 - a) * v * v;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            579,
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            43,
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1)
        );
    }
}
