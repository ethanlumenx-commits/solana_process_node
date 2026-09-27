use std::{str::FromStr};
use crate::rpc_client::{
    SolanaRpcClient
};

use crate::rpc_parse::{
    TransactionParse,
    InstructionParsed,
    DisPatcher,
    InstructionType,
};

use solana_pubkey::Pubkey;
use solana_signature::Signature;
use tracing::{error, info};


pub struct DataParse{
    rpcurl:String,
    client:SolanaRpcClient,
    dispatcher:DisPatcher,
}

impl DataParse{
    pub fn new(rpcurl:&str,) -> anyhow::Result<Self> {
        let helius_client = SolanaRpcClient::new(rpcurl.to_string());
        let dispatcher = DisPatcher::new()?;
        Ok(DataParse{
            rpcurl: rpcurl.to_string(), 
            client: helius_client,
            dispatcher,
        })
    }
    pub async fn get_newest_signatures_for_address(
        &self,
        pubkey:&Pubkey) -> anyhow::Result<Signature>{

            println!("RPCURL: {}", self.rpcurl);

            let signatures = self.client
                .get_signatures_for_address(pubkey).await?;

            let signature_info = signatures
                .first()
                .ok_or(anyhow::anyhow!("The signatures parse is empty"))?;
            
            let sig = Signature::from_str(&signature_info.signature)?;
            Ok(sig)
            
    }

    /// send one Signature and get transaction
    pub async fn get_transaction_with_config(
        &self,
        sig:&Signature,) -> anyhow::Result<TransactionParse>{

            let trasnaction = self.client
                .get_transaction_with_config(sig).await?;

            let transaction_parse = TransactionParse::new(trasnaction);

            Ok(transaction_parse)
    }
    pub async fn transaction_to_struct(
        &self,
        pubkey:&Pubkey) -> anyhow::Result<()>{

            let sig = self.get_newest_signatures_for_address(pubkey).await?;

            let transaction_parse = self.get_transaction_with_config(&sig).await?;
            
            // [xxx,xxx,xxx]
            let account_keys = transaction_parse
                .get_account_keys().ok_or(anyhow::anyhow!("The account_keys parse is error"))?;
            // [propgram_id,accounts,data,stack_height]
            let instructions = transaction_parse
                .get_instructions().ok_or(anyhow::anyhow!("The instructions parse is error"))?;

            for i in instructions.iter() { 
                // [propgram_id,accounts,data,stack_height] to InstructionParsed struct
                let parsed = InstructionParsed::from_ui_compiled_instruction(i, &account_keys)?;
                // info!("program_id: {}", parsed.program_id);
                // info!("accounts: {:?}", parsed.accounts);
                // info!("data: {}", parsed.data);
                // info!("dataded: {:?}",parsed.decode_data());
                // info!("stack_height: {:?}", parsed.stack_height);
                
                // dispatch by propgram_id 
                match self.dispatcher.dispatch(
                    &parsed,
                ) {
                    Ok(instruct_type)=>{
                        match instruct_type{
                            InstructionType::SystemTransfer(system_transfer)=>{
                                info!("SystemTransfer: {:?}", system_transfer);
                            }
                            InstructionType::ComputeBudget(compute_budget)=>{
                                info!("ComputeBudget: {:?}", compute_budget);
                            }
                            InstructionType::NotSupported=>{
                                info!("NotSupported");
                            }
                            
                        }
                    }
                    Err(e)=>{
                        error!("Error: {}", e);
                    }
                }

            }
            Ok(())
    }

}

#[cfg(test)]
mod tests { 
    use super::*;
    use dotenvy::dotenv;
    use crate::logger;

    #[tokio::test]
    async fn test_get_signatures_for_address(){ 
        dotenv().ok();
        let _guard = logger::init_logger();

        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        let pubkey = Pubkey::from_str("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg").unwrap();
        
        let parseclient = DataParse::new(&rpcurl);
        match parseclient{
            Ok(parseclient) => {
                let data = parseclient.transaction_to_struct(&pubkey).await;
                match data{
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        
    }
}