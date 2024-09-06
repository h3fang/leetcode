pub struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let m = flowerbed.len();
        for i in 0..m {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i + 1 == m || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                n -= 1;
                if n == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }
}
