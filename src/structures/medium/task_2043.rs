struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(mut balance: Vec<i64>) -> Self {
        balance.insert(0, 0);
        Bank { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.is_account_valid(account1, money) || !self.does_account_exist(account2) {
            false
        } else {
            self.balance[account1 as usize] -= money;
            self.balance[account2 as usize] += money;
            true
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.does_account_exist(account) {
            false
        } else {
            self.balance[account as usize] += money;
            true
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.is_account_valid(account, money) {
            false
        } else {
            self.balance[account as usize] -= money;
            true
        }
    }

    fn is_account_valid(&self, account: i32, money: i64) -> bool {
        self.does_account_exist(account) && self.balance[account as usize] >= money
    }

    fn does_account_exist(&self, account: i32) -> bool {
        (account as usize) < self.balance.len()
    }
}
