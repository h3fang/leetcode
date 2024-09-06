pub struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn is_account_valid(&self, acc: i32) -> bool {
        acc >= 1 && acc <= self.balance.len() as i32
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if self.is_account_valid(account1)
            && self.is_account_valid(account2)
            && money >= 0
            && self.balance[account1 as usize - 1] >= money
        {
            self.balance[account1 as usize - 1] -= money;
            self.balance[account2 as usize - 1] += money;
            true
        } else {
            false
        }
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.is_account_valid(account) || money < 0 {
            false
        } else {
            self.balance[account as usize - 1] += money;
            true
        }
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if self.is_account_valid(account)
            && money >= 0
            && self.balance[account as usize - 1] >= money
        {
            self.balance[account as usize - 1] -= money;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }
}
