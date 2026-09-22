use std::{str::FromStr};
use crate::rpc_client::solana_rpc_client::SolanaRpcClient;
use crate::rpc_parse::transaction_parse::TransactionParse;
use crate::rpc_parse::instruction_parse::InstructionParsed;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use tracing::info;


pub struct DataParse{
    rpcurl:String,
    client:SolanaRpcClient,
}

impl DataParse{
    pub fn new(rpcurl:&str,) -> Self {
        let helius_client = SolanaRpcClient::new(rpcurl.to_string());
        DataParse{
            rpcurl: rpcurl.to_string(), 
            client: helius_client
        }
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

            let account_keys = transaction_parse
                .get_account_keys().ok_or(anyhow::anyhow!("The account_keys parse is error"))?;

            let instructions = transaction_parse
                .get_instructions().ok_or(anyhow::anyhow!("The instructions parse is error"))?;

            info!("--------------------");
            for i in instructions.iter() { 
                let parsed = InstructionParsed::from_ui_compiled_instruction(i, &account_keys)?;
                info!("program_id: {}", parsed.program_id);
                info!("accounts: {:?}", parsed.accounts);
                info!("data: {}", parsed.data);
                info!("stack_height: {:?}", parsed.stack_height);
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
        let data = parseclient.transaction_to_struct(&pubkey).await;
        match data{
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}