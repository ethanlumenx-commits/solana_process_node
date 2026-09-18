use std::{any, thread::sleep, time::Duration};

use futures_util::future::ok;
use reqwest::Client;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

struct RPC{
    client: Arc<Client>,
    rpcurl: String
}

impl RPC{
    /// Create a new RPC instance and set the RPC URL
    /// timeout 10 and idle timeout 90
    fn new(rpcurl: String) -> Self{
        let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(90))
        .build()?;
        
        RPC{
            client: Arc::new(client),
            rpcurl
        }
    }
    /// Call the RPC endpoint with the given JSON
    async fn call<T:Serialize>(&self, json:&T) -> Result<Value,Error>{
        let a = json!({
            "jsonrpc": "2.0",
            "id": 1,
            methan:"GetAccountInfo",
            params:[],
        });
        const MAX_RETRIES: u8 = 5;
        // Retry up to MAX_RETRIES times
        for x in 0..MAX_RETRIES{
            let resp = self.client.post(&self.rpcurl)
            .json(json)
            .send().await;
            
            if let Err(e) = resp{
                if x == MAX_RETRIES-1{
                    return anyhow::bail!("RPC failed, Arrived MAX_RETRIE: {}", e);
                }
                sleep(Duration::from_millis(200 * (x + 1)as u64)).await;
                continue;
            }

            let body: Value = resp.json().await?;

            //  {
            // "jsonrpc":"2.0",
            // "id":1,
            // "error": {
            //     "code": -32602,
            //     "message":"Invalid params"
            //      }
            // }
            if let Some(err) = body.get("error"){
                return anyhow::bail!("RPC Error: {}", err);
            }

            // {
            // "jsonrpc":"2.0",
            // "id":1,
            // "result": {
            //     "context":{"slot":123},
            //     "value": {...}
            // }
            // }
            match body.get("result"){
                Some(result) => return Ok(result.clone()),
                None => continue,
            }
        }
        
        anyhow::bail!("RPC failed, Arrived MAX_RETRIE: {}", MAX_RETRIES);
        
    }
}