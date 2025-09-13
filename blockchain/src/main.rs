use std::collections::HashMap;

struct TokenSystem {
    ledger: HashMap<String, u64>,
}

impl TokenSystem {
    // Create a new token system
    fn new() -> Self {
        TokenSystem {
            ledger: HashMap::new(),
        }
    }

    // Create an account with initial balance
    fn create_account(&mut self, name: &str, balance: u64) {
        self.ledger.insert(name.to_string(), balance);
        println!("Account {} created with {} tokens.", name, balance);
    }

    // Check balance of an account
    fn get_balance(&self, name: &str) -> u64 {
        *self.ledger.get(name).unwrap_or(&0)
    }

    // Transfer tokens between accounts
    fn transfer(&mut self, from: &str, to: &str, amount: u64) {
        let from_balance = self.get_balance(from);

        if from_balance < amount {
            println!("❌ Transfer failed: {} has insufficient funds!", from);
            return;
        }

        // Deduct from sender
        self.ledger.insert(from.to_string(), from_balance - amount);

        // Add to receiver
        let to_balance = self.get_balance(to);
        self.ledger.insert(to.to_string(), to_balance + amount);

        println!("✅ {} sent {} tokens to {}", from, amount, to);
    }
}

fn main() {
    let mut system = TokenSystem::new();

    // Step 1: Create accounts
    system.create_account("Alice", 100);
    system.create_account("Bob", 50);

    // Step 2: Show balances
    println!("Alice balance: {}", system.get_balance("Alice"));
    println!("Bob balance: {}", system.get_balance("Bob"));

    // Step 3: Transfer tokens
    system.transfer("Alice", "Bob", 30);

    // Step 4: Show updated balances
    println!("Alice balance: {}", system.get_balance("Alice"));
    println!("Bob balance: {}", system.get_balance("Bob"));
}
