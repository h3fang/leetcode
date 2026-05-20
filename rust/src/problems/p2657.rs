pub struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let (mut x, mut y) = (0u64, 0u64);
        a.into_iter()
            .zip(b)
            .map(|(a, b)| {
                x |= 1 << a;
                y |= 1 << b;
                (x & y).count_ones() as i32
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
            vec![0, 2, 3, 4],
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![0, 1, 3],
            Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2])
        );
    }
}
