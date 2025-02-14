pub struct ProductOfNumbers {
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut prefix = Vec::with_capacity(1024);
        prefix.push(1);
        Self { prefix }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.resize(1, 0);
        } else {
            self.prefix.push(self.prefix.last().unwrap() * num);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix.len();
        if n > k as usize {
            self.prefix[n - 1] / self.prefix[n - k as usize - 1]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut p = ProductOfNumbers::new();
        p.add(3);
        p.add(0);
        p.add(2);
        p.add(5);
        p.add(4);
        assert_eq!(20, p.get_product(2));
        assert_eq!(40, p.get_product(3));
        assert_eq!(0, p.get_product(4));
        p.add(8);
        assert_eq!(32, p.get_product(2));
    }
}
