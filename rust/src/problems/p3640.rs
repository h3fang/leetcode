pub struct Solution;

const MIN: i64 = i64::MIN / 2;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut f = [MIN; 4];
        for w in nums.windows(2) {
            let (x, y) = (w[0] as i64, w[1] as i64);
            f[3] = if x < y { y + f[3].max(f[2]) } else { MIN };
            f[2] = if x > y { y + f[2].max(f[1]) } else { MIN };
            f[1] = if x < y { y + f[1].max(x) } else { MIN };
            f[0] = f[0].max(f[3]);
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(-4, Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::max_sum_trionic(vec![1, 4, 2, 7]));
    }
}
