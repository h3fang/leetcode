use std::collections::HashMap;

pub struct RangeFreqQuery {
    n: usize,
    t: Vec<HashMap<i32, i32>>,
}

#[allow(clippy::assigning_clones)]
impl RangeFreqQuery {
    pub fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut t = vec![HashMap::new(); arr.len() * 2];
        for (i, &a) in arr.iter().enumerate() {
            t[i + n].insert(a, 1);
        }
        for i in (0..n).rev() {
            t[i] = t[2 * i].clone();
            let kv = t[2 * i + 1].clone();
            for (k, v) in kv {
                *t[i].entry(k).or_default() += v;
            }
        }

        Self { n, t }
    }

    pub fn query(&mut self, left: i32, right: i32, value: i32) -> i32 {
        let mut l = left as usize + self.n;
        let mut r = right as usize + self.n;
        let mut result = 0;
        while l <= r {
            if l % 2 == 1 {
                result += self.t[l].get(&value).unwrap_or(&0);
                l += 1;
            }
            if r % 2 == 0 {
                result += self.t[r].get(&value).unwrap_or(&0);
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut rq = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(1, rq.query(1, 2, 4));
        assert_eq!(2, rq.query(0, 11, 33));
    }
}
