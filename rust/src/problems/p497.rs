use rand::prelude::*;

pub struct Solution {
    rng: ThreadRng,
    rects: Vec<Vec<i32>>,
    presum: Vec<i32>,
}

impl Solution {
    pub fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut presum = vec![0; rects.len() + 1];
        for (i, r) in rects.iter().enumerate() {
            let a = (r[2] - r[0] + 1) * (r[3] - r[1] + 1);
            presum[i + 1] = presum[i] + a;
        }
        Self {
            rng: rand::rng(),
            rects,
            presum,
        }
    }

    pub fn pick(&mut self) -> Vec<i32> {
        let sum = self.rng.random_range(0..*self.presum.last().unwrap());
        let i = self.presum.partition_point(|&x| x <= sum) - 1;
        let area = sum - self.presum[i];
        let r = &self.rects[i];
        let w = r[3] - r[1] + 1;
        let x = area / w + r[0];
        let y = area % w + r[1];
        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn case1() {
        let rects = vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]];
        let mut s = Solution::new(rects.clone());
        let mut freqs = HashMap::new();
        const N: usize = 10000;
        for _ in 0..N {
            let p = s.pick();
            *freqs.entry([p[0], p[1]]).or_insert(0) += 1;
        }

        let mut points = vec![];
        for r in rects {
            for x in r[0]..=r[2] {
                for y in r[1]..=r[3] {
                    let c = freqs.get(&[x, y]).cloned().unwrap_or(0);
                    let f = c as f64 / N as f64;
                    points.push(f);
                }
            }
        }
        let expected = 1.0 / points.len() as f64;
        println!("{points:?}");
        assert!(points.iter().all(|p| { (p - expected).abs() < 5e-2 }));
    }
}
