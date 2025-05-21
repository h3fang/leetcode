pub struct Solution;

fn digits_sum(mut x: i32) -> i32 {
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sums = Vec::with_capacity(n);
        for (i, &x) in nums.iter().enumerate() {
            sums.push((digits_sum(x), x, i as i32));
        }
        sums.sort_unstable();
        let mut cycles = 0;
        for mut i in 0..n as i32 {
            if sums[i as usize].2 < 0 {
                continue;
            }
            cycles += 1;
            while i >= 0 {
                let e = &mut sums[i as usize];
                i = e.2;
                e.2 = -1;
            }
        }
        n as i32 - cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_swaps(vec![37, 100]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_swaps(vec![22, 14, 33, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_swaps(vec![18, 43, 34, 16]));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::min_swaps(vec![268835996, 65052660, 415128775]));
    }
}
