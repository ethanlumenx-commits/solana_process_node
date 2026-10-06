use anyhow::Context;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, 
    EncodedTransaction, 
    UiCompiledInstruction, 
    UiInstruction, 
    UiMessage, 
    option_serializer::OptionSerializer
};
use tracing::info;
use crate::rpc_parse::instruction_parse::InstructionParsed;


// Transaction
// │
// ├── transaction // 我要执行什么
// │     │
// │     └── message
// │           │
// │           └── instructions
// │                  ↑
// │                  │
// │             用户直接提交
// │
// └── meta    // 执行之后发生了什么
//       │
//       ├── fee
//       ├── status
//       ├── pre_balances
//       ├── post_balances
//       ├── pre_token_balances
//       ├── post_token_balances
//       ├── log_messages
//       │
//       └── inner_instructions  // 程序在执行过程中又调用了哪些程序
//               ↑
//               │
//          程序执行过程中
//          CPI 产生的指令
pub struct TransactionParse{
    confirmed_transaction: EncodedConfirmedTransactionWithStatusMeta,
}

#[derive(Debug)]
pub struct InnerInstructionsParsed{
    pub index: usize,
    pub instructions: Vec<InstructionParsed>,
}

#[derive(Debug)]
pub struct InstructionWithInner {
    pub index: usize,
    pub instruction: InstructionParsed,
    pub inner_instructions: Vec<InstructionParsed>,
}



impl TransactionParse{
    /// create and move transaction
    pub fn new(
        transaction:EncodedConfirmedTransactionWithStatusMeta)->Self{
            TransactionParse{confirmed_transaction: transaction}
    }

    pub fn get_slot(&self)->u64{
        self.confirmed_transaction.slot
    }

    pub fn get_block_time(&self)->Option<i64>{
        self.confirmed_transaction.block_time
    }

    /// Transaction -> transaction -> enum::Json(UiTransaction), 
    /// -> pub message: UiMessage, -> enum ::Raw(UiRawMessage), 
    /// -> pub account_keys: Vec<String>,
    pub fn get_account_keys(&self) -> Option<Vec<String>> {
        if let EncodedTransaction::Json(tx) =
            &self.confirmed_transaction.transaction.transaction
        {
            if let UiMessage::Raw(raw) = &tx.message {
                let mut account_keys = raw.account_keys.clone();

                // add loaded addresses from meta
                if let Some(meta) =
                    &self.confirmed_transaction.transaction.meta{
                    match meta.loaded_addresses.as_ref(){
                        OptionSerializer::Some(loaded_addresses) =>{
                            account_keys.extend(loaded_addresses.writable.clone());
                            account_keys.extend(loaded_addresses.readonly.clone());
                        }
                        OptionSerializer::None =>{}
                        OptionSerializer::Skip =>{}
                    }
                }

                return Some(account_keys);
            }
        }

        None
    }

    /// Transaction -> transaction -> enum::Json(UiTransaction), 
    /// -> pub message: UiMessage, -> enum ::Raw(UiRawMessage), 
    /// -> pub instructions: Vec<UiCompiledInstruction>,
    /// pub struct UiCompiledInstruction 
    ///     pub program_id_index: u8,
    ///     pub accounts: Vec<u8>,
    ///     pub data: String,
    ///     pub stack_height: Option<u32>,
    pub fn get_instructions(&self)->Option<Vec<UiCompiledInstruction>>{


        if let EncodedTransaction::Json(tx) = &self.confirmed_transaction.transaction.transaction{ 
            if let UiMessage::Raw(raw) = &tx.message{ 
                return Some(raw.instructions.clone());
            }
        }
        None
    }

    pub fn get_inner_instructions(&self)->anyhow::Result<Vec<InnerInstructionsParsed >>{
        let meta = self.confirmed_transaction.transaction.meta.as_ref()
            .ok_or_else(||anyhow::anyhow!("No meta"))?;
        // info!("fee: {}", meta.fee);
        // info!("compute_units: {:?}",meta.compute_units_consumed);

        let accounts = self.get_account_keys()
            .ok_or_else(||anyhow::anyhow!("No account keys from get_meat"))?;
        let mut inner_instructions_list = Vec::new();
        
        match &meta.inner_instructions{
            OptionSerializer::Some(inner_instructions) =>{
                // info!("Top inner_instructions len: {}",inner_instructions.len());
                
                // 遍历每一层 instruction，看有几个顶层 instruction 发出的指令的 CPI 指令
                for inner in inner_instructions.iter(){
                    // info!("Top inner instruction accounts index:{}",inner.index);
                    // info!("instructions len: {}",inner.instructions.len());
                    
                    // 每个指令下涉及的 CPI 指令
                    let mut ui_instructions = Vec::new();
                    for instruction in inner.instructions.iter(){
                        if let UiInstruction::Compiled(ui) = instruction{ 
                            ui_instructions.push(InstructionParsed::from_ui_compiled_instruction(ui, &accounts)?);
                            // info!("program_id_index: {}",ui.program_id_index);
                            // info!("accounts: {:?}",ui.accounts);
                            // info!("data: {}",ui.data);
                            // info!("stack_height: {:?}",ui.stack_height);
                        }
                    }
                    inner_instructions_list.push(
                        InnerInstructionsParsed{
                            index: inner.index as usize,
                            instructions: ui_instructions,
                        }
                    );
                }
            }
            OptionSerializer::None =>{}
            OptionSerializer::Skip =>{}
        }
        Ok(inner_instructions_list)
        
    }

    pub fn get_instructions_with_inner_instructions(&self)->anyhow::Result<Vec<InstructionWithInner>>{
        let instructions = self.get_instructions()
            .ok_or_else(||anyhow::anyhow!("No instructions by get_instructions_with_inner_instructions"))?;
        let accounts = self.get_account_keys()
            .ok_or_else(||anyhow::anyhow!("No account keys by get_instructions_with_inner_instructions"))?;
        let inner_instructions = self.get_inner_instructions()
            .context("get_inner_instructions err")?;
        
        let mut result = Vec::new();

        for (index, data) in instructions.iter().enumerate() { 
            let parse = InstructionParsed::from_ui_compiled_instruction(data, &accounts)?;
            
            let inner = inner_instructions
                .iter()
                .find(|inner| inner.index == index)
                .map(|inner| inner.instructions.clone())
                .unwrap_or_default();

            result.push(InstructionWithInner{
                index,
                instruction: parse,
                inner_instructions: inner,
            });
        }

        Ok(result)
    }


}

#[cfg(test)]
mod tests { 
    use super::*;
    use std::{ str::FromStr};
    use dotenvy::dotenv;
    use crate::rpc_client::solana_rpc_client::SolanaRpcClient;
    use solana_pubkey::Pubkey;
    use solana_signature::Signature;
    use crate::logger;

    #[tokio::test]
    async fn test_get_account_keys() { 
        dotenv().ok();
        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        println!("RPCURL: {}", rpcurl);

        let helius_client = SolanaRpcClient::new(rpcurl);
        let pubkey = Pubkey::from_str("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg").unwrap();

        let signatures = helius_client.get_signatures_for_address(&pubkey).await.unwrap();
        println!("Signatures's len: {}", signatures.len());
        for signature in signatures.iter().take(10) {
            println!("Signature: {}", signature.signature);
            let sig = Signature::from_str(&signature.signature);
            if sig.is_err() {
                println!("Signature error: {}", sig.err().unwrap());
                continue;
            }
            if let Ok(sig) = sig { 
                let transaction = helius_client.get_transaction_with_config(&sig).await.unwrap();
                println!("Transaction: {:?}", transaction);
                let transaction_parse = TransactionParse::new(transaction);
                let account_keys = transaction_parse.get_account_keys();
                println!("Account Keys: {:?}", account_keys);
            }
        }
    }

    #[tokio::test]
    async fn test_get_meta() ->anyhow::Result<()> { 
        dotenv().ok();
        
        let _guard = logger::init_logger();
        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        println!("RPCURL: {}", rpcurl);

        let helius_client = SolanaRpcClient::new(rpcurl);
        let pubkey = Pubkey::from_str("GF8SKKobum6UJnhX2mLHePU38htg5vdr9zcY4jH8Pqs2").unwrap();

        let signatures = helius_client.get_signatures_for_address(&pubkey).await.unwrap();
        println!("Signatures's len: {}", signatures.len());
        let sig = Signature::from_str(signatures[0].signature.as_str()).unwrap();

        let transaction = helius_client.get_transaction_with_config(&sig).await.unwrap();

        let transaction_parse = TransactionParse::new(transaction);
        // let resp = transaction_parse.get_inner_instructions()?;
        // info!("resp: {:?}", resp);

        let resp = transaction_parse.get_instructions_with_inner_instructions()?;
        for item in resp.iter(){ 
            info!("Top level index: {}",item.index);
            info!("Top level instruction: {:?}",item.instruction);
            info!("Top level inner_instructions len: {}",item.inner_instructions.len());
            for inner in item.inner_instructions.iter(){ 
                info!("inner{:?}",inner);
            }
            info!("#################");
        }
        Ok(())
        
    }
}