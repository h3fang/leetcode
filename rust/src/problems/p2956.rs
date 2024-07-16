pub struct Solution;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut f1, mut f2) = ([0; 101], [0; 101]);
        let (mut c1, mut c2) = (0, 0);
        for &x in &nums1 {
            f1[x as usize] += 1;
        }
        for &x in &nums2 {
            f2[x as usize] += 1;
            if f1[x as usize] > 0 {
                c2 += 1;
            }
        }
        for &x in &nums1 {
            if f2[x as usize] > 0 {
                c1 += 1;
            }
        }
        vec![c1, c2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 4],
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6])
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 0],
            Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5])
        )
    }
}
