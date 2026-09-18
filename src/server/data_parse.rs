use std::{str::FromStr};
use crate::rpc_client::solana_rpc_client::SolanaRpcClient;
use crate::rpc_parse::transaction_parse::TransactionParse;
use solana_pubkey::Pubkey;
use solana_signature::Signature;


pub struct DateParse;

impl DateParse{
    pub async fn get_signatures_for_address(
        rpcurl:&str,
        pubkey:&str) -> anyhow::Result<()>{

            println!("RPCURL: {}", rpcurl);
            let helius_client = SolanaRpcClient::new(rpcurl.to_string());
            let pubkey = Pubkey::from_str(pubkey)?;

            let signatures = helius_client.get_signatures_for_address(&pubkey).await?;
            
            let sig = Signature::from_str(&signatures[0].signature)?;
            let transaction = helius_client.get_transaction_with_config(&sig).await?;
            let transaction_parse = TransactionParse::new(transaction);
            let account_keys = transaction_parse.get_account_keys();
            println!("Account Keys: {:?}", account_keys);
            Ok(())
    }

}

#[cfg(test)]
mod tests { 
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_signatures_for_address() { 
        dotenv().ok();
        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        let pubkey = "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg";
        let data = DateParse::get_signatures_for_address(&rpcurl, &pubkey).await;
        match data{
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}