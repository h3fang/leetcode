pub struct Solution;

impl Solution {
    pub fn shifting_letters(mut s: String, shifts: Vec<Vec<i32>>) -> String {
        let b = unsafe { s.as_bytes_mut() };
        let n = b.len();
        let mut diff = vec![0; n];
        for e in shifts {
            let d = if e[2] == 1 { 1 } else { -1 };
            diff[e[0] as usize] += d;
            if e[1] as usize + 1 < n {
                diff[e[1] as usize + 1] -= d;
            }
        }
        let mut shift = 0;
        for (b, d) in b.iter_mut().zip(&diff) {
            shift += d;
            *b = ((((*b - b'a') as i32 + shift) % 26 + 26) % 26) as u8 + b'a';
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abc".to_string();
        let shifts = [[0, 1, 0], [1, 2, 1], [0, 2, 1]];
        let shifts = shifts.iter().map(|s| s.to_vec()).collect();
        assert_eq!("ace", Solution::shifting_letters(s, shifts));
    }

    #[test]
    fn case2() {
        let s = "dztz".to_string();
        let shifts = [[0, 0, 0], [1, 1, 1]];
        let shifts = shifts.iter().map(|s| s.to_vec()).collect();
        assert_eq!("catz", Solution::shifting_letters(s, shifts));
    }
}
