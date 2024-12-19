pub struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        height
            .iter()
            .enumerate()
            .take(height.len() - 1)
            .filter_map(|(i, &x)| {
                if x > threshold {
                    Some(i as i32 + 1)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut r = Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2);
        r.sort_unstable();
        assert_eq!(vec![3, 4], r);
    }

    #[test]
    fn case2() {
        let mut r = Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3);
        r.sort_unstable();
        assert_eq!(vec![1, 3], r);
    }

    #[test]
    fn case3() {
        let r = Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10);
        assert!(r.is_empty());
    }
}
