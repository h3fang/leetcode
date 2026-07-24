pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        let max = *nums.last().unwrap();
        let max = 1usize << (i32::BITS - max.leading_zeros());

        let mut pairs = vec![false; max];
        pairs[0] = true;
        for (i, &x) in nums.iter().enumerate() {
            for &y in &nums[i + 1..] {
                pairs[(x ^ y) as usize] = true;
            }
        }

        let mut triplets = vec![false; max];

        for x in nums {
            for (y, &b) in pairs.iter().enumerate() {
                if b {
                    triplets[(x ^ y as i32) as usize] = true;
                }
            }
        }

        triplets.into_iter().filter(|&b| b).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::unique_xor_triplets(vec![1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::unique_xor_triplets(vec![6, 7, 8, 9]));
    }
}
