pub struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut f = [0u64; 3];
        let mut count = 0;
        a.into_iter()
            .zip(b)
            .map(|(x, y)| {
                f[0] |= 1 << x;
                f[1] |= 1 << y;
                for z in [x, y] {
                    if f[2] & (1 << z) == 0 && f[0] & (1 << z) > 0 && f[1] & (1 << z) > 0 {
                        f[2] |= 1 << z;
                        count += 1;
                    }
                }
                count
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
