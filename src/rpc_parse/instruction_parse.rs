use std::{str::FromStr};

use anyhow::Result;
use bs58::decode;
use solana_pubkey::Pubkey;
use solana_transaction_status_client_types::UiCompiledInstruction;

// instructions: [
// UiCompiledInstruction { program_id_index: 3, accounts: [], data: "3b1H8Rq1T3d1", stack_height: Some(1)}, 
// UiCompiledInstruction { program_id_index: 3, accounts: [], data: "LKoyXd", stack_height: Some(1)},
// UiCompiledInstruction { program_id_index: 2, accounts: [0,1], data: "3Bxs4ThwQbE4vyj5", stack_height: Some(1)}
// ]
#[derive(Debug,Clone)]
pub struct InstructionParsed {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: String,
    pub decode_data: Vec<u8>,
    pub stack_height: Option<u32>,
}

impl InstructionParsed{
    pub fn from_ui_compiled_instruction(
        ui_compiled_instruction: &UiCompiledInstruction,
        accounts:&[String]
    ) -> anyhow::Result<Self> {
        let program_id_string = 
            accounts.get(ui_compiled_instruction.program_id_index as usize)
            .ok_or_else(|| anyhow::anyhow!("Invaid program_id"))?;
        let program_id = Pubkey::from_str(program_id_string)?;
        
        let accounts = ui_compiled_instruction.accounts
            .iter()
            .map(|accounts_index|{
                let account = accounts.get(*accounts_index as usize)
                .ok_or_else(|| anyhow::anyhow!("Invalid account index = {}, accounts_len = {}", accounts_index, accounts.len()))?;

                Ok(Pubkey::from_str(account)?)
                
            })
            .collect::<Result<Vec<Pubkey>>>()?;

        let data = ui_compiled_instruction.data.clone();

        let decode_data = decode(&data).into_vec()?;

        let stack_height = ui_compiled_instruction.stack_height;
            
        Ok(Self {
            program_id,
            accounts,
            data,
            decode_data,
            stack_height,
        })
    }

    pub fn decode_data(&self)->anyhow::Result<Vec<u8>>{
        
        let decoded_data = decode(&self.data).into_vec()?;
        Ok(decoded_data)
    }
}