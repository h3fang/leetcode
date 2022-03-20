pub struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut c_t = [0; 7];
        let mut c_b = [0; 7];
        let mut c_all = [0; 7];
        for (&t, &b) in tops.iter().zip(&bottoms) {
            c_t[t as usize] += 1;
            c_b[b as usize] += 1;
            if t == b {
                c_all[t as usize] += 1;
            } else {
                c_all[t as usize] += 1;
                c_all[b as usize] += 1;
            }
        }
        let n = tops.len();
        for i in 1..=6 {
            if c_all[i] == n {
                let c = n - c_t[i].max(c_b[i]);
                result = result.min(c as i32);
            }
        }
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        assert_eq!(2, Solution::min_domino_rotations(tops, bottoms));
    }

    #[test]
    fn case2() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        assert_eq!(-1, Solution::min_domino_rotations(tops, bottoms));
    }
}
