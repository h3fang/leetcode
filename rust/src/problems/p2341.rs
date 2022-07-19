pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut f = vec![0; 101];
        for n in nums {
            f[n as usize] += 1;
        }
        let mut pairs = 0;
        let mut rem = 0;
        for e in f {
            pairs += e / 2;
            rem += e % 2;
        }
        vec![pairs, rem]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 1],
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 0], Solution::number_of_pairs(vec![1, 1]));
    }
}
