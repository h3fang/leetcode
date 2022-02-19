pub struct Solution;

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for max in (1..=arr.len() as i32).rev() {
            let k = arr.iter().position(|e| *e == max).unwrap() as i32;
            if k + 1 != max {
                result.push(k + 1);
                arr[..k as usize + 1].reverse();
                result.push(max);
                arr[..max as usize].reverse();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pancake_sort(mut arr: Vec<i32>) {
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        let result = Solution::pancake_sort(arr.clone());
        for k in result {
            arr[..k as usize].reverse();
        }
        assert_eq!(sorted, arr);
    }

    #[test]
    fn case1() {
        test_pancake_sort(vec![3, 2, 4, 1]);
    }

    #[test]
    fn case2() {
        test_pancake_sort(vec![1, 2, 3]);
    }
}
