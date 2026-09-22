use std::{str::FromStr};

use anyhow::Result;
use solana_pubkey::Pubkey;
use solana_transaction_status_client_types::UiCompiledInstruction;

pub struct InstructionParsed {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: String,
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
                .ok_or_else(|| anyhow::anyhow!("Invalid account"))?;

                Ok(Pubkey::from_str(account)?)
                
            })
            .collect::<Result<Vec<Pubkey>>>()?;

        let data = ui_compiled_instruction.data.clone();

        let stack_height = ui_compiled_instruction.stack_height;
            


        Ok(Self {
            program_id,
            accounts,
            data,
            stack_height,
        })
    }
}