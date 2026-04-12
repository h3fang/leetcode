pub struct Solution;

use std::sync::OnceLock;

static DIST: OnceLock<[[i32; 26]; 26]> = OnceLock::new();

fn init() -> [[i32; 26]; 26] {
    let mut dist = [[0; 26]; 26];
    for (i, r) in dist.iter_mut().enumerate() {
        let (x0, y0) = (i / 6, i % 6);
        for (j, c) in r.iter_mut().enumerate() {
            let (x1, y1) = (j / 6, j % 6);
            *c = (x0.abs_diff(x1) + y0.abs_diff(y1)) as i32;
        }
    }
    dist
}

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let dist = DIST.get_or_init(init);

        let mut f = [[0; 26]; 2];

        for w in word.as_bytes().windows(2) {
            let (x, y) = ((w[0] - b'A') as usize, (w[1] - b'A') as usize);
            let d = &dist[x];
            for another in 0..26 {
                f[1][another] = (f[0][another] + d[y]).min(f[0][y] + d[another]);
            }
            f.swap(0, 1);
        }

        *f[0].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_distance("CAKE".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::minimum_distance("HAPPY".to_string()));
    }
}
