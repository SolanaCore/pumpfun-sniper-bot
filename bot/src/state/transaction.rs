use solana_program::{
    instruction::Instruction,
    pubkey::Pubkey,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>, // Placeholder for data, can be extended later
}

impl Transaction {
    pub fn get_program_id(&self) -> &Pubkey {
        &self.program_id
    }

    pub fn get_accounts(&self) -> &Vec<Pubkey> {
        &self.accounts
    }
    
    pub fn get_data(&self) -> Vec<u8> {
        &self.data
    }

    pub fn place_trade(&self) -> Result<Trade, Error> {
        //TODO - 1
        // calculate buy_amount and sell_amount 
        let trade_acc = Trade {
            trade_id: self.get_program_id().to_string(),
            instruction: Instruction {
                program_id: *self.get_program_id(),
                accounts: self.get_accounts().clone(),
                data: self.get_data(),
            },
            buy_amount: 0, // Placeholder, should be set based on actual logic
            sell_amount: 0, // Placeholder, should be set based on actual logic
        }

        trade_acc.buy_token().unwrap(); // Assuming buy_token returns Result<Trade, Error>
        trade_acc.sell_token().unwrap(); // Assuming sell_token returns Result<Trade, Error>
    }
}


/*
Accounts(pump create ix):
MintWritableSigner	
Mint Authority	
Bonding CurveWritable	
Associated Bonding CurveWritable	
Global	
Mpl Token Metadata	
Token Metadata Program
MetadataWritable	
UserWritableSigner	
System Program	
System Program
Token Program	
Token Program
Associated Token Program	
Associated Token Program
Rent	
Sysvar: Rent
Event Authority	
Program	
*/
/*
Data(pump create ix):
Name
string	
Symbol
Uri	
Creator
*/
//Program id  -> 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
