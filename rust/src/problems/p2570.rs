pub struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (nums1.len(), nums2.len());
        let mut ans = Vec::with_capacity(m + n);
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            match nums1[i][0].cmp(&nums2[j][0]) {
                std::cmp::Ordering::Less => {
                    ans.push(nums1[i].clone());
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    let sum = nums1[i][1] + nums2[j][1];
                    ans.push(vec![nums1[i][0], sum]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => {
                    ans.push(nums2[j].clone());
                    j += 1;
                }
            }
        }
        if i < m {
            ans.extend(nums1.into_iter().skip(i));
        } else if j < n {
            ans.extend(nums2.into_iter().skip(j));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = [[1, 2], [2, 3], [4, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let nums2 = [[1, 4], [3, 2], [4, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let expected = [[1, 6], [2, 3], [3, 2], [4, 6]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::merge_arrays(nums1, nums2));
    }

    #[test]
    fn case2() {
        let nums1 = [[2, 4], [3, 6], [5, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let nums2 = [[1, 3], [4, 3]].iter().map(|v| v.to_vec()).collect();
        let expected = [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::merge_arrays(nums1, nums2));
    }
}
