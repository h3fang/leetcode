pub struct Solution;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut pre = 0u64;
        let mut suf = [0; 51];
        nums.iter().for_each(|&x| suf[x as usize] += 1);
        nums.into_iter()
            .map(|x| {
                pre |= 1 << x;
                suf[x as usize] -= 1;
                pre.count_ones() as i32 - suf.iter().filter(|&&e| e > 0).count() as i32
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
            vec![-3, -1, 1, 3, 5],
            Solution::distinct_difference_array(vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-2, -1, 0, 2, 3],
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2])
        );
    }
}
