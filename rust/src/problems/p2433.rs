pub struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; pref.len()];
        result[0] = pref[0];
        for (i, p) in pref.iter().enumerate().skip(1) {
            result[i] = pref[i - 1] ^ p;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![5, 7, 2, 3, 2],
            Solution::find_array(vec![5, 2, 0, 3, 1])
        );
    }
}
