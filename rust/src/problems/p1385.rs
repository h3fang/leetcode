pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(mut arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr1.sort_unstable();
        arr2.sort_unstable();

        let (mut ans, mut j) = (0, 0);
        let n = arr2.len();

        for x in arr1 {
            while j < n && arr2[j] < x - d {
                j += 1;
            }

            if j == n || arr2[j] > x + d {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6)
        );
    }
}
