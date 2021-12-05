pub struct Solution;

impl Solution {
    pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
        fn bt(digits: &[i32], used: &mut [bool], i: usize, curr: i32, result: &mut Vec<i32>) {
            if i == 3 {
                result.push(curr);
            } else {
                for (j, &d) in digits.iter().enumerate() {
                    if used[j]
                        || (j > 0 && !used[j - 1] && digits[j - 1] == d)
                        || (i == 0 && d == 0)
                        || (i == 2 && d % 2 != 0)
                    {
                        continue;
                    }
                    used[j] = true;
                    bt(digits, used, i + 1, curr * 10 + d, result);
                    used[j] = false;
                }
            }
        }
        let mut result = Vec::new();
        let mut used = vec![false; digits.len()];
        digits.sort_unstable();
        bt(&digits, &mut used, 0, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
            Solution::find_even_numbers(vec![2, 1, 3, 0])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![200], Solution::find_even_numbers(vec![0, 2, 0, 0]));
    }
}
