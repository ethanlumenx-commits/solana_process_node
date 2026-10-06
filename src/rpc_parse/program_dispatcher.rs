use std::str::FromStr;
use tracing::info;
use solana_pubkey::Pubkey;
use crate::rpc_parse::{
    InstructionParsed, 
    SystemTransfer,
    ComputeBudgetInstruction,
    TokenTransfer,
    TokenTransferChecked,
};

pub struct DisPatcher{
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub compute_budget_program: Pubkey,
}

#[derive(Debug)]
pub enum InstructionType{
    SystemTransfer(SystemTransfer),
    ComputeBudget(ComputeBudgetInstruction),
    TokenProgram(TokenTransfer),
    TokenTransferChecked(TokenTransferChecked),
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
                info!("ComputeBudgetInstruction: {:?}", data);
                Ok(InstructionType::ComputeBudget(data))
            }

            p if p == self.token_program =>{
                if instruction.decode_data.len() < 9{ 
                    return Err(anyhow::anyhow!("Invalid token_program data length{}",instruction.decode_data.len()));
                }
                if instruction.accounts.len() < 3{ 
                    return Err(anyhow::anyhow!("Invalid token_program accounts length{}",instruction.accounts.len()));
                }
                match instruction.decode_data[0]{
                    3=>{
                        let token_transfer = TokenTransfer::new(
                            &instruction.accounts,
                            &instruction.decode_data,
                        )?;
                        info!("TokenTransfer: {:?}", token_transfer);
                        Ok(InstructionType::TokenProgram(token_transfer))
                    }
                    12=>{
                        let token_transfer_checked = TokenTransferChecked::new(
                            &instruction.accounts,
                            &instruction.decode_data,
                        )?;
                        info!("TokenTransferChecked: {:?}", token_transfer_checked);
                        Ok(InstructionType::TokenTransferChecked(token_transfer_checked))
                    }
                    _=>{
                        return Err(anyhow::anyhow!("Not Matched token_program instruction type {}", instruction.decode_data[0]))
                    }
                }

                
            }
            
            _=>{
                Err(anyhow::anyhow!("Not Matched instruction program_id:{}",instruction.program_id))
            }
        }
        
    }
}