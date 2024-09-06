pub struct Bitset {
    bits: Vec<bool>,
    cnt: i32,
    reversed: bool,
}

impl Bitset {
    pub fn new(size: i32) -> Self {
        Self {
            bits: vec![false; size as usize],
            cnt: 0,
            reversed: false,
        }
    }

    pub fn fix(&mut self, idx: i32) {
        let idx = idx as usize;
        if !(self.bits[idx] ^ self.reversed) {
            self.bits[idx] = !self.bits[idx];
            self.cnt += 1;
        }
    }

    pub fn unfix(&mut self, idx: i32) {
        let idx = idx as usize;
        if self.bits[idx] ^ self.reversed {
            self.bits[idx] = !self.bits[idx];
            self.cnt -= 1;
        }
    }

    pub fn flip(&mut self) {
        self.reversed = !self.reversed;
        self.cnt = self.bits.len() as i32 - self.cnt;
    }

    pub fn all(&self) -> bool {
        self.cnt == self.bits.len() as i32
    }

    pub fn one(&self) -> bool {
        self.cnt > 0
    }

    pub fn count(&self) -> i32 {
        self.cnt
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.bits.len());
        for b in &self.bits {
            if *b ^ self.reversed {
                result.push('1');
            } else {
                result.push('0');
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
        let mut bs = Bitset::new(5);
        bs.fix(3);
        bs.fix(1);
        bs.flip();
        assert!(!bs.all());
        bs.unfix(0);
        bs.flip();
        assert!(bs.one());
        bs.unfix(0);
        assert_eq!(2, bs.count());
        assert_eq!("01010", bs.to_string());
    }
}
