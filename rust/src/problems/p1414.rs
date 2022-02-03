pub struct Solution;

impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut nums = vec![1, 1];
        let mut a = 1;
        let mut b = 1;
        while a + b <= k {
            let c = a + b;
            nums.push(c);
            a = b;
            b = c;
        }
        let mut result = 0;
        let mut i = nums.len() - 1;
        while k > 0 {
            if nums[i] <= k {
                k -= nums[i];
                result += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::find_min_fibonacci_numbers(7));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::find_min_fibonacci_numbers(10));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::find_min_fibonacci_numbers(19));
    }
}
