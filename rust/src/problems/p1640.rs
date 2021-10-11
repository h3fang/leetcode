pub struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut used = vec![false; pieces.len()];
        let mut j = 0;
        while j < arr.len() {
            let mut found = false;
            for (i, state) in used.iter_mut().enumerate() {
                if !*state && pieces[i][0] == arr[j] {
                    for k in &pieces[i] {
                        if *k == arr[j] {
                            j += 1;
                        } else {
                            return false;
                        }
                    }
                    found = true;
                    *state = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            false,
            Solution::can_form_array(vec![1, 2, 3], vec![vec![2], vec![1, 3]])
        );
    }
}
