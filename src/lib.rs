use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen,env};

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    solution: String
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            solution
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_guess(&mut self, solution: String){
        self.solution = solution ;
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        if self.solution == solution{
            env::log_str("Correct");
            true
        }
        else {
            env::log_str("Not Found");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn debug_get_hash(){
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "this is my solution";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("lets dubug:{:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution(){
        let ben = AccountId::new_unchecked("ben.testnet".to_string());
        let context = get_context(ben);
        testing_env!(context.build());
        let mut contract = Contract::new("28a011e40add5bb2e0630ebdc73ec9bec41054fc3afdfa7899461bd4eb002413".to_string());
        let mut guess_result =contract.guess_solution("wrong answer here".to_string());
        assert!(!guess_result, "expected a failure from the guess result");
        assert_eq!(get_logs(), ["Not Found"], "expected a failure log");
        guess_result = contract.guess_solution("this is my solution".to_string());
        assert!(guess_result, "expected a correct answer to return");
        assert_eq!(get_logs(), ["Not Found", "Correct"]," expected a success log after previous fail");
    }
}
