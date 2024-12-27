pub struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut pos = Vec::with_capacity(nums.len());
        for (i, &y) in nums.iter().enumerate() {
            if y == x {
                pos.push(i as i32);
            }
        }
        queries
            .into_iter()
            .map(|q| {
                if q <= pos.len() as i32 {
                    pos[q as usize - 1]
                } else {
                    -1
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
        assert_eq!(
            vec![0, -1, 2, -1],
            Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1],
            Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5)
        );
    }
}
