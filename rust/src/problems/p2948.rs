pub struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut v = nums
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<Vec<_>>();
        v.sort_unstable();
        let n = nums.len();
        let mut result = vec![0; n];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && (v[j].0 - v[j - 1].0) <= limit {
                j += 1;
            }
            let mut pos = v[i..j].iter().map(|e| e.1).collect::<Vec<_>>();
            pos.sort_unstable();
            for (x, k) in v[i..j].iter().map(|e| e.0).zip(pos) {
                result[k] = x;
            }
            i = j;
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
            vec![1, 3, 5, 8, 9],
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 6, 7, 18, 1, 2],
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![1, 7, 28, 19, 10],
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3)
        );
    }
}
