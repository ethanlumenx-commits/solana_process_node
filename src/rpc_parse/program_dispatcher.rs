use std::str::FromStr;
use tracing::info;
use solana_pubkey::Pubkey;
use crate::rpc_parse::{
    InstructionParsed, 
    SystemTransfer,
    ComputeBudgetInstruction,
};

pub struct DisPatcher{
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub compute_budget_program: Pubkey,
}

pub enum InstructionType{
    SystemTransfer(SystemTransfer),
    ComputeBudget(ComputeBudgetInstruction),
    NotSupported,
}

impl DisPatcher{
    pub fn new()->anyhow::Result<Self>{
        Ok(Self{
            system_program: Pubkey::from_str("11111111111111111111111111111111")?,
            token_program: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,
            compute_budget_program: Pubkey::from_str("ComputeBudget111111111111111111111111111111")?,
        })
    }

    pub fn dispatch(
        &self,
        instruction:&InstructionParsed,
    )->anyhow::Result<InstructionType>{
        match instruction.program_id{
            p if p == self.system_program =>{
                if instruction.decode_data.len() < 12{ 
                    return Err(anyhow::anyhow!("Invalid system_program data length{}",instruction.decode_data.len()));
                }
                let instruction_type = u32::from_le_bytes(instruction.decode_data[0..4].try_into()?);
                match instruction_type{
                    2=>{
                        let system_transfer = SystemTransfer::new(
                            &instruction.accounts,
                            &instruction.decode_data,
                        )?;
                        info!("SystemTransfer: {:?}", system_transfer);
                        Ok(InstructionType::SystemTransfer(system_transfer) )
                    }
                    _=>{
                        Ok(InstructionType::NotSupported)
                    }

                }
            }
            
            p if p == self.compute_budget_program =>{
                let data = ComputeBudgetInstruction::new(&instruction.decode_data)?;
                Ok(InstructionType::ComputeBudget(data))
            }
            _=>{
                Err(anyhow::anyhow!("Not Matched instruction program_id"))
            }
        }
        
    }
}