pub struct Solution;

impl Solution {
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        let mut nums2 = nums2
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e, i as i32))
            .collect::<Vec<_>>();
        nums2.sort_unstable_by_key(|e| e.0);
        let mut i = 0;
        let mut j = 0;
        let mut result = vec![-1; nums1.len()];
        let mut remaining = Vec::with_capacity(nums1.len());
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j].0 {
                result[nums2[j].1 as usize] = nums1[i];
                i += 1;
                j += 1;
            } else {
                remaining.push(nums1[i]);
                i += 1;
            }
        }
        let mut i = 0;
        for r in remaining {
            while i < result.len() {
                if result[i] == -1 {
                    result[i] = r;
                    i += 1;
                    break;
                }
                i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn advantage(a: &[i32], b: &[i32]) -> usize {
        a.iter().zip(b).filter(|(c, d)| c > d).count()
    }

    #[test]
    fn case1() {
        let result = Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]);
        assert_eq!(4, advantage(&result, &[1, 10, 4, 11]))
    }

    #[test]
    fn case2() {
        let result = Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]);
        assert_eq!(3, advantage(&result, &[13, 25, 32, 11]))
    }
}
