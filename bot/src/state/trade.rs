use srede::{
    Serialize, Deserialize
}
use crate::state::Instruction;

#[derive(Debug,Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: String,
    pub instruction: Instruction,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub slippage: f64,
    pub priority_fee: u64,
}

impl Trade {
    pub fn buy_token(self, ) -> Result<()> {
        const BUY_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];

        //we will use most of the accounts from the create ix along with the accounts that buy ix specificially needs
        //            ..self.instruction.accounts.clone()

        let buy_accounts  = Buy {
        }
        /*
        amount: u64, // Amount of tokens to buy
        max_sol_cost: u64,
        */
        // adjust the token_out as per the slippage set...
        let buy_args = BuyInstructionArgs {
            amount: self.token_amount, // Placeholder, should be set based on actual logic
            max_sol_cost: self.sol_amount, // Placeholder, should be set based on actual logic
        }

        let buy_instruction: solana_program::instruction::Instruction = buy_accounts.instruction(buy_args);

        let mut transaction = Transaction::new_with_payer(&[buy_instruction], /* payer */ );
        Ok(())
    }

    pub fn sell_token(self, ) -> Result<()> {
        const BUY_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
        //we will use most of the accounts from the create ix along with the accounts that sell ix specificially needs
        let sell_accounts  = Sell {
        }
        /*
        amount: u64, // Amount of tokens to buy
        min_sol_output: u64,
        */
        // adjust the token_out as per the slippage set...
        let buy_args = SellInstructionArgs {
            amount: self.token_amount, // Placeholder, should be set based on actual logic
            min_sol_output: self.sol_amount, // Placeholder, should be set based on actual logic
        }

        let buy_instruction: solana_program::instruction::Instruction = sell_accounts.instruction(buy_args);

        let mut transaction = Transaction::new_with_payer(&[buy_instruction], /* payer */ );
        Ok(())
    }
}
