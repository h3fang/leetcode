use rand::prelude::*;

pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    pub fn pick(&self, target: i32) -> i32 {
        let mut result = 0;
        for (i, (j, _)) in self
            .nums
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == target)
            .enumerate()
        {
            let r = thread_rng().gen_range(0..=i);
            if r == 0 {
                result = j;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = Solution::new(vec![1, 2, 3, 3, 3]);
        assert!([2, 3, 4].contains(&s.pick(3)));
        assert_eq!(0, s.pick(1));
        let mut count = [0; 5];
        let n = 30000i32;
        for _ in 0..n {
            count[s.pick(3) as usize] += 1;
        }
        assert!(count[0] == 0);
        assert!(count[1] == 0);
        for i in [2, 3, 4] {
            assert!((count[i] - n / 3).abs() <= n / 10);
        }
    }
}
