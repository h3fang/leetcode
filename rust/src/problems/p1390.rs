pub struct Solution;

use std::sync::OnceLock;

static STAT: OnceLock<(Vec<i32>, Vec<i32>)> = OnceLock::new();

fn init() -> (Vec<i32>, Vec<i32>) {
    let mut count = vec![0; 10_0001];
    let mut sum = vec![0; 10_0001];
    for i in 1..10_0001 {
        for j in (i..10_0001).step_by(i) {
            count[j] += 1;
            sum[j] += i as i32;
        }
    }
    (count, sum)
}

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let (count, sum) = STAT.get_or_init(init);
        let mut ans = 0;
        for x in nums {
            if count[x as usize] == 4 {
                ans += sum[x as usize];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(32, Solution::sum_four_divisors(vec![21, 4, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(64, Solution::sum_four_divisors(vec![21, 21]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]));
    }
}
