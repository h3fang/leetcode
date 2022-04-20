pub struct ATM {
    count: [i64; 5],
}

impl ATM {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { count: [0; 5] }
    }

    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (a, b) in self.count.iter_mut().zip(banknotes_count) {
            *a += b as i64;
        }
    }

    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount as i64;
        let mut result = vec![0; 5];
        for ((&note, r), &c) in [20, 50, 100, 200, 500]
            .iter()
            .zip(result.iter_mut())
            .zip(&self.count)
            .rev()
        {
            *r = (amount / note).min(c) as i32;
            amount -= note * (*r) as i64;
        }
        if amount > 0 {
            vec![-1]
        } else {
            for (a, b) in self.count.iter_mut().zip(&result) {
                *a -= *b as i64;
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(vec![0, 0, 1, 0, 1], atm.withdraw(600));
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(vec![-1], atm.withdraw(600));
        assert_eq!(vec![0, 1, 0, 0, 1], atm.withdraw(550));
    }
}
