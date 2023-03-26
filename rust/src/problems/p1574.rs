pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut j = n - 1;
        while j > 0 && arr[j - 1] <= arr[j] {
            j -= 1;
        }
        if j == 0 {
            return 0;
        }
        let mut result = j;
        for (i, &a) in arr.iter().enumerate() {
            while j < n && arr[j] < a {
                j += 1;
            }
            result = result.min(j - i - 1);
            if i + 1 < n && a > arr[i + 1] {
                break;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::find_length_of_shortest_subarray(vec![1, 2, 3]));
    }
}
