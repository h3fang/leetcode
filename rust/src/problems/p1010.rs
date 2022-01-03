pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = [0; 60];
        let mut result = 0;
        for t in time {
            let t = (t % 60) as usize;
            result += count[(60 - t) % 60];
            count[t] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::num_pairs_divisible_by60(vec![60, 60, 60]));
    }
}
