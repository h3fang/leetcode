pub struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut freq = [0; 10];
        for d in digits {
            freq[d as usize] += 1;
        }

        let mut ans = 0;
        let kinds = freq.iter().filter(|&&f| f != 0).count() as i32;
        let non_zeros = kinds - i32::from(freq[0] > 0);
        let singles = freq.iter().skip(1).filter(|&&f| f == 1).count() as i32;

        for (d, &c) in freq.iter().enumerate().step_by(2) {
            if c == 0 {
                continue;
            }

            let second = kinds - i32::from(c == 1);
            let third = non_zeros - i32::from(d > 0 && c == 1);
            let mut invalid = singles;
            if d > 0 {
                if c == 1 {
                    invalid -= 1;
                } else if c == 2 {
                    invalid += 1;
                }
            }
            ans += second * third - invalid;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::total_numbers(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::total_numbers(vec![0, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::total_numbers(vec![6, 6, 6]));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::total_numbers(vec![1, 3, 5]));
    }
}
