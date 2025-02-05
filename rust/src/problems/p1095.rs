pub struct MountainArray {
    arr: Vec<i32>,
}

impl MountainArray {
    pub fn new(arr: Vec<i32>) -> Self {
        Self { arr }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }

    pub fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn find_in_mountain_array(target: i32, arr: &MountainArray) -> i32 {
        fn find(mut target: i32, arr: &MountainArray, mut l: i32, mut r: i32, sign: i32) -> i32 {
            target *= sign;
            while l <= r {
                let m = (r - l) / 2 + l;
                match (sign * arr.get(m)).cmp(&target) {
                    std::cmp::Ordering::Less => l = m + 1,
                    std::cmp::Ordering::Equal => return m,
                    std::cmp::Ordering::Greater => r = m - 1,
                }
            }
            -1
        }
        let n = arr.length();
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            let m = (r - l) / 2 + l;
            if arr.get(m) < arr.get(m + 1) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        let a = find(target, arr, 0, l, 1);
        if a == -1 {
            find(target, arr, l, n - 1, -1)
        } else {
            a
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::find_in_mountain_array(3, &MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1]))
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::find_in_mountain_array(3, &MountainArray::new(vec![0, 1, 2, 4, 2, 1]))
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::find_in_mountain_array(2, &MountainArray::new(vec![1, 5, 2]))
        );
    }
}
