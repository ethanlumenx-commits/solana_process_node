use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, 
    EncodedTransaction, 
    //UiTransactionEncoding,
    UiMessage,
};

pub struct TransactionParse{
    confirmed_transaction: EncodedConfirmedTransactionWithStatusMeta,
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

    /// Transaction -> transaction -> enum::Json(UiTransaction), -> pub message: UiMessage, -> enum ::Raw(UiRawMessage), -> pub account_keys: Vec<String>,
    pub fn get_account_keys(&self)->Vec<String>{
        if let EncodedTransaction::Json(tx) = &self.confirmed_transaction.transaction.transaction{ 
            if let UiMessage::Raw(raw) = &tx.message{ 
                return raw.account_keys.clone();
            }
        }
        Vec::new()
    }


}

#[cfg(test)]
mod tests { 
    use super::*;
    use std::{str::FromStr};
    use dotenvy::dotenv;
    use crate::rpc_client::solana_rpc_client::SolanaRpcClient;
    use solana_pubkey::Pubkey;
    use solana_signature::Signature;

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
}