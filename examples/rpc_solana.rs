use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_commitment_config::CommitmentConfig;
use std::str::FromStr;
use anyhow::Result;


#[tokio::main]
async fn main()->Result<()>{
    let client = RpcClient::new_with_commitment(
        String::from("https://mainnet.helius-rpc.com/?api-key=95c412ce-fbaf-4468-a543-9065b3af2578"),
        CommitmentConfig::confirmed()
    );

    let pubkey = Pubkey::from_str("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg")?;

    let balance = client.get_balance(&pubkey).await?;

    println!("Balance: {}", balance);
    
    let account = client.get_account(&pubkey).await?;
    let lamp = account.lamports;
    println!("Account: {:?}", account);
    println!("Lamp: {}", lamp);
    Ok(())
}