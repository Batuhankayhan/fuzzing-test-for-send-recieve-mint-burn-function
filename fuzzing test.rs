use proptest::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TokenContract {
    balances: HashMap<String, u64>,
    total_supply: u64,
}

impl TokenContract {
    fn mint(&mut self, to: &str, amount: u64) {
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        self.total_supply += amount;
    }

    fn burn(&mut self, from: &str, amount: u64) {
        if let Some(balance) = self.balances.get_mut(from) {
            if *balance >= amount {
                *balance -= amount;
                self.total_supply -= amount;
            }
        }
    }

    fn send(&mut self, from: &str, to: &str, amount: u64) {
        if let Some(balance) = self.balances.get_mut(from) {
            if *balance >= amount {
                *balance -= amount;
                *self.balances.entry(to.to_string()).or_insert(0) += amount;
            }
        }
    }

    fn balance_of(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }
}

proptest! {
    #[test]
    fn fuzz_mint_send_burn(sender in "[a-z]{1,10}", receiver in "[a-z]{1,10}", mint_amount in 1u64..1_000_000, transfer_amount in 0u64..1_000_000, burn_amount in 0u64..1_000_000) {
        let mut contract = TokenContract::default();

        contract.mint(&sender, mint_amount);
        prop_assert_eq!(contract.balance_of(&sender), mint_amount);

        contract.send(&sender, &receiver, transfer_amount);
        let sender_balance = contract.balance_of(&sender);
        let receiver_balance = contract.balance_of(&receiver);
        prop_assert!(sender_balance + receiver_balance <= mint_amount);

        contract.burn(&sender, burn_amount);
        prop_assert!(contract.balance_of(&sender) <= sender_balance);
    }
}
