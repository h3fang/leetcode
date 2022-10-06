pub struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let ones = arr.iter().sum::<i32>() as usize;
        if ones % 3 != 0 {
            return vec![-1, -1];
        }
        if ones == 0 {
            return vec![0, 2];
        }
        let mut i1 = 0;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut c = 0;
        for (i, e) in arr.iter().enumerate() {
            if *e == 0 {
                continue;
            }
            c += 1;
            if c == 1 {
                i1 = i;
            } else if c == 1 + ones / 3 {
                i2 = i;
            } else if c == 1 + 2 * ones / 3 {
                i3 = i;
            }
        }
        let len = arr.len() - i3;

        if i1 + len <= i2
            && i2 + len <= i3
            && arr[i1..i1 + len] == arr[i3..]
            && arr[i2..i2 + len] == arr[i3..]
        {
            vec![(i1 + len - 1) as i32, (i2 + len) as i32]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![0, 3], Solution::three_equal_parts(vec![1, 0, 1, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1],
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(vec![0, 2], Solution::three_equal_parts(vec![1, 1, 0, 0, 1]));
    }
}
