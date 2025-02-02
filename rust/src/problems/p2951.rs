pub struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        mountain
            .windows(3)
            .enumerate()
            .filter_map(|(i, w)| {
                if w[1] > w[0] && w[1] > w[2] {
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
        assert!(Solution::find_peaks(vec![2, 4, 4]).is_empty());
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 3], Solution::find_peaks(vec![1, 4, 3, 8, 5]));
    }
}
