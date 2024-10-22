pub struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut right = [0; 3];
        for (i, s) in garbage.iter().enumerate() {
            result += s.len();
            for &c in s.as_bytes() {
                match c {
                    b'M' => right[0] = i,
                    b'P' => right[1] = i,
                    b'G' => right[2] = i,
                    _ => {}
                }
            }
        }
        (1..travel.len()).for_each(|i| travel[i] += travel[i - 1]);
        let mut result = result as i32;
        for r in right {
            result += if r == 0 { 0 } else { travel[r - 1] };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let garbage = ["G", "P", "GP", "GG"];
        let travel = vec![2, 4, 3];
        let garbage = garbage.iter().map(|g| g.to_string()).collect();
        assert_eq!(21, Solution::garbage_collection(garbage, travel));
    }

    #[test]
    fn case2() {
        let garbage = ["MMM", "PGM", "GP"];
        let travel = vec![3, 10];
        let garbage = garbage.iter().map(|g| g.to_string()).collect();
        assert_eq!(37, Solution::garbage_collection(garbage, travel));
    }
}
