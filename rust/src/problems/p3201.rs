pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut f = [[0, 0]; 2];
        for x in nums {
            let x = (x % 2) as usize;
            for y in 0..2 {
                f[y][x] = f[x][y] + 1;
            }
        }
        f.into_iter().flatten().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::maximum_length(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::maximum_length(vec![1, 3]));
    }
}
