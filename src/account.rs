use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Account {
    // client_id: u16, 
    // total - held
    pub available: f32,

    // total - available
    pub held: f32,
   
    // available + held
    pub total: f32,
   
    // account is locked
    pub locked: bool,
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
    pub fn deposit(mut self, amount: f32) -> Self {
        self.available += amount;
        self.update_account();
        self
    }

    pub fn withdrawal(mut self, amount: f32) -> Self {
        if amount <= self.available {
            self.available -= amount;
            self.update_account();
            
        }
        else {
            // insufficient amount
        }
        self
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

