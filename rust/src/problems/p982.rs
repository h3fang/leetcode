pub struct Solution;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut m = [0; 1 << 16];
        for &a in &nums {
            for &b in &nums {
                m[(a & b) as usize] += 1;
            }
        }
        let mut result = 0;
        for &c in &nums {
            let x = c ^ 0xffff;
            let mut sub = x;
            while sub > 0 {
                result += m[sub as usize];
                sub = (sub - 1) & x;
            }
            result += m[0];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::count_triplets(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(27, Solution::count_triplets(vec![0, 0, 0]));
    }
}
