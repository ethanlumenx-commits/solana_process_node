use solana_rpc_client::nonblocking::rpc_client::{RpcClient};
use solana_pubkey::Pubkey;
use solana_commitment_config::CommitmentConfig;

use anyhow::Result;
use std::sync::Arc;


use solana_signature::Signature;
use solana_transaction_status_client_types::{
    UiTransactionEncoding,
    EncodedConfirmedTransactionWithStatusMeta,
};
use solana_rpc_client_types::response::RpcConfirmedTransactionStatusWithSignature;
use solana_rpc_client_types::config::RpcTransactionConfig;

use solana_account::Account;
use solana_hash::Hash;

pub struct HeliusRpcClient { 
    rpc_client: Arc<RpcClient>,
}

impl HeliusRpcClient{
    pub fn new(rpc_url: String) -> Self{
        let rpc_client = Arc::new(
            RpcClient::new_with_commitment(
                rpc_url,
                CommitmentConfig::confirmed())
        );
        HeliusRpcClient{rpc_client}
    }

    /// Get balance
    /// Balance: u64
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64>{
        let balance = self.rpc_client.get_balance(pubkey).await?;
        Ok(balance)
    }

    /// Get account info
    /// Account { lamports: 39869364, data.len: 0, owner: 11111111111111111111111111111111, executable: false, rent_epoch: 18446744073709551615 }
    pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account>{
        let account = self.rpc_client.get_account(pubkey).await?;
        Ok(account)
    }

    /// Get multiple accounts, send a slice of pubkeys
    pub async fn get_multiple_accounts(&self, pubkey: &[Pubkey]) -> Result<Vec<Option<Account>>>{
        let account = self.rpc_client.get_multiple_accounts(pubkey).await?;
        Ok(account)
    }

    /// Get transaction
    /// Transaction { signatures: [...], message: Message { ... } }
    pub async fn get_transaction(&self, signature: &Signature) -> Result<EncodedConfirmedTransactionWithStatusMeta>{
        let transaction = self.rpc_client.get_transaction(signature,UiTransactionEncoding::Base64).await?;
        Ok(transaction)
    }

    /// Get Transaction Status with  support the Transaction version(0)
    /// Get transaction with some config
    pub async fn get_transaction_with_config(&self, signature: &Signature) -> Result<EncodedConfirmedTransactionWithStatusMeta>{ 
        let config = RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig::confirmed()), //processed 刚看到数据    confirmed 网络已经确认    finalized 最终确定
            max_supported_transaction_version: Some(0),  // 客户端最多支持 Version 0 的交易
        };

        let transaction = self.rpc_client.get_transaction_with_config(
            &signature,
            config,
        ).await?;

        Ok(transaction)

    }

    /// get latest blcok hash
    pub async fn get_latest_blockhash(&self) -> Result<Hash>{
        let block_hash = self.rpc_client.get_latest_blockhash().await?;
        Ok(block_hash)
    }
    /// Get signatures for address
    pub async fn get_signatures_for_address(&self, address:&Pubkey) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>>{ 
        let signature = self.rpc_client.get_signatures_for_address(address).await?;
        Ok(signature)
    }

    pub async fn get_block_height(&self) -> Result<u64>{
        let block_height = self.rpc_client.get_block_height().await?;
        Ok(block_height)
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    use std::{str::FromStr, sync::Arc};
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_balance() { 
        dotenv().ok();
        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        println!("RPCURL: {}", rpcurl);
        let helius_client = Arc::new(HeliusRpcClient::new(rpcurl));
        let block_height = helius_client.get_block_height().await.unwrap();
        println!("Block Height: {}", block_height);
    }

    // Pubkey  
    // ↓
    // get_signatures_for_address
    // ↓
    // Signature
    // ↓
    // get_transaction
    // ↓
    // Transaction
    #[tokio::test]
    async fn test_stream_transaction() { 
        dotenv().ok();
        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        println!("RPCURL: {}", rpcurl);
        let helius_client = Arc::new(HeliusRpcClient::new(rpcurl));
        
        // Pubkey
        let pubkey = Pubkey::from_str("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg").unwrap();
        println!("Pubkey: {}", pubkey);
        // get_signatures_for_address
        // it will return a vector of signatures about 1000 transactions
        let signatures = helius_client.get_signatures_for_address(&pubkey).await.unwrap();

        // just take the first 10 signatures
        // it will refercence the signature
        for signature in signatures.iter().take(10) {
            println!("Signature: {}", signature.signature);
            // String to Signature
            let sig = Signature::from_str(&signature.signature);
            if sig.is_err() {
                println!("Signature error: {}", sig.err().unwrap());
                continue;
            }
            // get the transaction
            if let Ok(sig) = sig { 
                let transaction = helius_client.get_transaction_with_config(&sig).await.unwrap();
                println!("Transaction: {:?}", transaction);
            }
            
            
        }
    }

}