pub struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut f = vec![0; value as usize];

        nums.into_iter().for_each(|e| {
            let e = e.rem_euclid(value);
            f[e as usize] += 1;
        });

        let (i, min) = f
            .into_iter()
            .enumerate()
            .min_by_key(|e| (e.1, e.0))
            .unwrap();
        min * value + i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7)
        );
    }
}
