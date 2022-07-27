pub struct Solution;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let max = *nums.iter().max().unwrap();
        let mut exp = 1;
        let mut buff = vec![0; n];
        while max / exp > 0 {
            let mut count = [0; 10];
            for &e in &nums {
                let i = ((e / exp) % 10) as usize;
                count[i] += 1;
            }
            for i in 1..10 {
                count[i] += count[i - 1];
            }
            for &e in nums.iter().rev() {
                let i = ((e / exp) % 10) as usize;
                buff[count[i] - 1] = e;
                count[i] -= 1;
            }
            std::mem::swap(&mut nums, &mut buff);
            exp *= 10;
        }

        let mut result = 0;
        for w in nums.windows(2) {
            result = result.max(w[1] - w[0]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_gap(vec![3, 6, 9, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::maximum_gap(vec![3]));
    }
}
