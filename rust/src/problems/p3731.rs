pub struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let mut f = 0u128;
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for &x in &nums {
            f |= 1 << x;
            min = min.min(x);
            max = max.max(x);
        }

        nums.clear();
        for x in min + 1..max {
            if f & (1 << x) == 0 {
                nums.push(x);
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3], Solution::find_missing_elements(vec![1, 4, 2, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_missing_elements(vec![7, 8, 6, 9])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![2, 3, 4], Solution::find_missing_elements(vec![5, 1]));
    }
}
