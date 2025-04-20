pub struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut freq = [0; 1000];
        for x in answers {
            freq[x as usize] += 1;
        }
        freq.into_iter()
            .enumerate()
            .map(|(g, f)| {
                let g = g as i32 + 1;
                (f + g - 1) / g * g
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::num_rabbits(vec![1, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::num_rabbits(vec![10, 10, 10]));
    }
}
