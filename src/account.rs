use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Account {
    // client_id: u16, 
    // total - held
    available: f32,

    // total - available
    held: f32,
   
    // available + held
    total: f32,
   
    // account is locked
    locked: bool,
}

impl Account {
    pub fn new(available: f32) -> Self {
        // TODO validate arguments if necessary 
            
        Account{
            // client_id,
            available,
            held: 0.0,
            total: available,
            locked: false,
        }
    }

    fn update_account(mut self) {
        self.total = self.available + self.held;
    }

    // deposit amount to account 
    pub fn deposit(mut self, amount: f32) {
        // assert!(amount > 0.0);
        if amount > 0.0 {
            self.available += amount;
            self.update_account();
        }
    }

    pub fn withdrawal(mut self, amount: f32) -> bool {
        if amount <= self.available {
            self.available -= amount;
            self.update_account();
            true
        }
        else {
            // insufficient amount
            false
        }
    }

    pub fn dispute(mut self, amount: f32) {
        self.held += amount;
        self.available -= amount;
        self.update_account();
    }

    pub fn resolve(mut self, amount: f32) {
        self.held -= amount;
        self.available += amount;
        self.update_account();
    }


}

