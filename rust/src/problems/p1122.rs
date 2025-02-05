pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut idx = [1001; 1001];
        for (i, x) in arr2.into_iter().enumerate() {
            idx[x as usize] = i;
        }
        arr1.sort_unstable_by_key(|&x| (idx[x as usize], x));
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![22, 28, 8, 6, 17, 44],
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
        )
    }
}
