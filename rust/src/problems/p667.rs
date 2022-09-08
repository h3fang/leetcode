pub struct Solution;

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        for i in 1..n - k {
            result.push(i);
        }
        let mut i = n - k;
        let mut j = n;
        while i <= j {
            result.push(i);
            if i != j {
                result.push(j);
            }
            i += 1;
            j -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(n: i32, k: i32, arr: &[i32]) {
        let mut m = vec![false; n as usize];
        for w in arr.windows(2) {
            let a = (w[0] - w[1]).abs();
            m[a as usize] = true;
        }
        assert_eq!(k, m.iter().filter(|e| **e).count() as i32);
    }

    #[test]
    fn case1() {
        let n = 3;
        let k = 1;
        assert_valid(n, k, &Solution::construct_array(n, k));
    }
}
