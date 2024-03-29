struct Bank {
    balance: Vec<i64>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize;
        let account2 = account2 as usize;
        
        if account1 > self.balance.len() || account2 > self.balance.len() {
            return false;
        }
        
        if self.balance[account1-1] < money {
            return false;
        }
        
        self.balance[account1-1] -= money;
        self.balance[account2-1] += money;
        
        true
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize;
        if account > self.balance.len() {
            return false;
        }
        
        self.balance[account-1] += money;
        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize;
        if account > self.balance.len() {
            return false;
        }
        
        if self.balance[account-1] < money {
            return false;
        }
        
        self.balance[account-1] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
