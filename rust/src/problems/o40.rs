use rand::prelude::*;

pub struct Solution;

impl Solution {
    pub fn get_least_numbers(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        fn partition(arr: &mut [i32], left: usize, right: usize, rng: &mut ThreadRng) -> usize {
            arr.swap(rng.random_range(left..=right), right);
            let p = arr[right];
            let mut i = left;
            for k in left..right {
                if arr[k] <= p {
                    arr.swap(k, i);
                    i += 1;
                }
            }
            arr.swap(i, right);
            i
        }

        fn select(arr: &mut [i32], left: usize, right: usize, rng: &mut ThreadRng, k: usize) {
            if left >= right {
                return;
            }
            let mid = partition(arr, left, right, rng);
            let n = mid - left + 1;
            match n.cmp(&k) {
                std::cmp::Ordering::Less => select(arr, mid + 1, right, rng, k - n),
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => select(arr, left, mid - 1, rng, k),
            }
        }
        let n = arr.len();
        select(&mut arr, 0, n - 1, &mut rand::rng(), k as usize);
        arr[..k as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::get_least_numbers(vec![3, 2, 1], 2);
        result.sort_unstable();
        assert_eq!(vec![1, 2], result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::get_least_numbers(vec![0, 1, 2, 1], 1);
        result.sort_unstable();
        assert_eq!(vec![0], result);
    }

    #[test]
    fn case3() {
        let mut result = Solution::get_least_numbers(vec![0, 0, 2, 3, 2, 1, 1, 2, 0, 4], 10);
        result.sort_unstable();
        assert_eq!(vec![0, 0, 0, 1, 1, 2, 2, 2, 3, 4], result);
    }

    #[test]
    fn case4() {
        let mut result = Solution::get_least_numbers(
            vec![
                0, 1, 1, 1, 4, 5, 3, 7, 7, 8, 10, 2, 7, 8, 0, 5, 2, 16, 12, 1, 19, 15, 5, 18, 2, 2,
                22, 15, 8, 22, 17, 6, 22, 6, 22, 26, 32, 8, 10, 11, 2, 26, 9, 12, 9, 7, 28, 33, 20,
                7, 2, 17, 44, 3, 52, 27, 2, 23, 19, 56, 56, 58, 36, 31, 1, 19, 19, 6, 65, 49, 27,
                63, 29, 1, 69, 47, 56, 61, 40, 43, 10, 71, 60, 66, 42, 44, 10, 12, 83, 69, 73, 2,
                65, 93, 92, 47, 35, 39, 13, 75,
            ],
            75,
        );
        result.sort_unstable();
        assert_eq!(
            vec![
                0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
                7, 7, 8, 8, 8, 8, 9, 9, 10, 10, 10, 10, 11, 12, 12, 12, 13, 15, 15, 16, 17, 17, 18,
                19, 19, 19, 19, 20, 22, 22, 22, 22, 23, 26, 26, 27, 27, 28, 29, 31, 32, 33, 35, 36,
                39, 40, 42
            ],
            result
        );
    }
}
