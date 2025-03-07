pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let (mut max_f, mut n) = (0, 0);
        let mut freq = [0; 101];
        for x in nums {
            freq[x as usize] += 1;
            match freq[x as usize].cmp(&max_f) {
                std::cmp::Ordering::Equal => n += 1,
                std::cmp::Ordering::Greater => {
                    max_f = freq[x as usize];
                    n = 1;
                }
                _ => {}
            }
        }
        max_f * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]));
    }
}
