pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut a, mut ca, mut b, mut cb) = (0, 0, 0, 0);
        for &x in &nums {
            if x == a && ca > 0 {
                ca += 1;
            } else if x == b && cb > 0 {
                cb += 1;
            } else if ca == 0 {
                a = x;
                ca = 1;
            } else if cb == 0 {
                b = x;
                cb = 1;
            } else {
                ca -= 1;
                cb -= 1;
            }
        }
        let mut result = vec![];
        if ca > 0 {
            let f = nums.iter().filter(|&&x| x == a).count();
            if f * 3 > n {
                result.push(a);
            }
        }
        if cb > 0 {
            let f = nums.iter().filter(|&&x| x == b).count();
            if f * 3 > n {
                result.push(b);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3], Solution::majority_element(vec![3, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1], Solution::majority_element(vec![1]));
    }

    #[test]
    fn case3() {
        assert_eq!(vec![1, 2], Solution::majority_element(vec![1, 2]));
    }
}
