const MOD: i64 = 10_0000_0007;

fn qpow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n >>= 1;
    }
    ans
}

pub struct Fancy {
    vals: Vec<i64>,
    inv: Vec<i64>,
    add: i64,
    mul: i64,
    inv_mul: i64,
}

impl Default for Fancy {
    fn default() -> Self {
        let inv = (0..=100).map(|i| qpow(i, MOD - 2)).collect();
        Self {
            vals: Vec::with_capacity(10000),
            inv,
            add: 0,
            mul: 1,
            inv_mul: 1,
        }
    }
}

impl Fancy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, val: i32) {
        let v = (val as i64 - self.add + MOD) * self.inv_mul;
        self.vals.push(v % MOD);
    }

    pub fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }

    pub fn mult_all(&mut self, m: i32) {
        self.add = (self.add * m as i64) % MOD;
        self.mul = (self.mul * m as i64) % MOD;
        self.inv_mul = (self.inv_mul * self.inv[m as usize]) % MOD;
    }

    pub fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        self.vals
            .get(idx)
            .map(|v| ((v * self.mul) % MOD + self.add) % MOD)
            .unwrap_or(-1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut f = Fancy::new();
        f.append(2);
        f.add_all(3);
        f.append(7);
        f.mult_all(2);
        assert_eq!(10, f.get_index(0));
        f.add_all(3);
        f.append(10);
        f.mult_all(2);
        assert_eq!(26, f.get_index(0));
        assert_eq!(34, f.get_index(1));
        assert_eq!(20, f.get_index(2));
    }
}
