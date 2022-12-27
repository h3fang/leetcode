pub struct Solution;

impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let (mut i, mut j, mut result, mut total) = (1, 1, 0, 0);
        while total + result + i <= n {
            result += i;
            total += result;
            i += 1;
        }
        while total < n {
            result += 1;
            total += j;
            j += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_boxes(3));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_boxes(4));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::minimum_boxes(10));
    }
}
