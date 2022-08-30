pub struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut count = vec![vec![(0, 0); 0]; 3];
        for (i, s) in garbage.iter().enumerate() {
            let mut m = 0;
            let mut p = 0;
            let mut g = 0;
            for &c in s.as_bytes() {
                match c {
                    b'M' => m += 1,
                    b'P' => p += 1,
                    b'G' => g += 1,
                    _ => {}
                }
            }
            if m > 0 {
                count[0].push((m, i));
            }
            if p > 0 {
                count[1].push((p, i));
            }
            if g > 0 {
                count[2].push((g, i));
            }
        }
        let mut result = 0;
        for kind in count {
            let mut curr = 0;
            for (n, i) in kind {
                result += n;
                result += travel[curr..i].iter().sum::<i32>();
                curr = i;
            }
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
