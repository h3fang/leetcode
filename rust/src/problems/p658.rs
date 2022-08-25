pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut right = arr.partition_point(|&e| e <= x) as i32;
        let mut left = right - 1;
        for _ in 0..k {
            if left < 0 {
                right += 1;
            } else if right >= arr.len() as i32 || x - arr[left as usize] <= arr[right as usize] - x
            {
                left -= 1;
            } else {
                right += 1;
            }
        }
        arr[(left + 1) as usize..right as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1)
        );
    }
}
