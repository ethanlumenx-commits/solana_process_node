
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
use tracing::{error,info};


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

    pub async fn get_1000_signatures_for_address(
        &self,
        pubkey:&Pubkey) -> anyhow::Result<Vec<Signature>>{

            let signatures = self.client
                .get_signatures_for_address(pubkey).await?;

            let signatures_info = signatures
                .iter()
                .filter_map(|s|{
                    let sig = Signature::from_str(&s.signature).ok();
                    return sig;
                })
                .collect();
            Ok(signatures_info)
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
        pubkey:&Pubkey) -> anyhow::Result<Vec<InstructionType> >{

            let sig = self.get_newest_signatures_for_address(pubkey).await?;

            let transaction_parse = self.get_transaction_with_config(&sig).await?;
            
            // [xxx,xxx,xxx]
            let account_keys = transaction_parse
                .get_account_keys().ok_or(anyhow::anyhow!("The account_keys parse is error"))?;
            // [propgram_id,accounts,data,stack_height]
            let instructions = transaction_parse
                .get_instructions().ok_or(anyhow::anyhow!("The instructions parse is error"))?;

            let mut args = Vec::new();
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
                        args.push(instruct_type);

                    }
                    Err(e)=>{
                        error!("Error: {}", e);
                    }
                }
            
            }
            Ok(args)
    }

    pub async fn signature_to_struct(
        &self,
        sig:&Signature) -> anyhow::Result<Vec<InstructionType> >{

            let transaction_parse = self.get_transaction_with_config(sig).await?;
            
            // [xxx,xxx,xxx]
            let account_keys = transaction_parse
                .get_account_keys().ok_or(anyhow::anyhow!("The account_keys parse is error"))?;
            // [propgram_id,accounts,data,stack_height]
            let instructions = transaction_parse
                .get_instructions().ok_or(anyhow::anyhow!("The instructions parse is error"))?;

            let mut args = Vec::new();
            for i in instructions.iter() { 
                // [propgram_id,accounts,data,stack_height] to InstructionParsed struct
                let parsed = InstructionParsed::from_ui_compiled_instruction(i, &account_keys)?;

                // dispatch by propgram_id 
                match self.dispatcher.dispatch(
                    &parsed,
                ) {
                    Ok(instruct_type)=>{
                        args.push(instruct_type);

                    }
                    Err(e)=>{
                        error!("Error: {}", e);
                    }
                }
            
            }
            Ok(args)
    }

    pub async fn transaction_to_struct_1000(
        &self,
        pubkey:&Pubkey) -> anyhow::Result<Vec<Vec<InstructionType> > >{

            let sigs = self.get_1000_signatures_for_address(pubkey).await?;
            info!("sigs len: {}", sigs.len());
            let mut args = Vec::new();
            for i in sigs.iter().take(10){
                let transaction_parse = self.get_transaction_with_config(i).await?;
                let account_keys = transaction_parse
                    .get_account_keys().ok_or(anyhow::anyhow!("The account_keys parse is error"))?;
                let instructions = transaction_parse
                    .get_instructions().ok_or(anyhow::anyhow!("The instructions parse is error"))?;
                
                let mut arg = Vec::new();
                info!("---------------------------");
                for i in instructions.iter() { 
                    let parsed = InstructionParsed::from_ui_compiled_instruction(i, &account_keys)?;
                    info!("program_id: {}", parsed.program_id);
                    info!("accounts: {:?}", parsed.accounts);
                    info!("data: {}", parsed.data);
                    info!("dataded: {:?}",parsed.decode_data());
                    info!("stack_height: {:?}", parsed.stack_height);
                    
                    match self.dispatcher.dispatch(
                        &parsed,
                    ) {
                        Ok(instruct_type)=>{
                            arg.push(instruct_type);
                        }
                        Err(e)=>{
                            error!("Error: {}", e)
                        }
                    }
                }
                args.push(arg);
            }

            Ok(args)
            

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
                    Ok(e) => {
                        println!("Success");
                        e.iter().for_each(|f|{
                            match f{
                                InstructionType::SystemTransfer(e) => println!("SystemTransfer: {:?}", e),
                                InstructionType::ComputeBudget(e) => println!("ComputeBudget: {:?}", e),
                                InstructionType::NotSupported => println!("NotSupported"),
                                _ => (),
                            }
                        })
                    },
                    Err(err) => println!("Error: {}", err),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        
    }


    #[tokio::test]
    async fn test_get_1000_signatures_for_address() -> anyhow::Result<()> {
        dotenv().ok();
        let _guard = logger::init_logger();

        let rpcurl = std::env::var("RPCURL")
            .expect("RPC_URL not set");

        let pubkey = Pubkey::from_str(
            "GF8SKKobum6UJnhX2mLHePU38htg5vdr9zcY4jH8Pqs2"
        )?;

        let parseclient = DataParse::new(&rpcurl)?;

        let _data = parseclient
            .transaction_to_struct_1000(&pubkey)
            .await?;

        println!("Success");

        Ok(())
    }

    #[tokio::test]
    async fn test_signatures_for_address(){ 
        dotenv().ok();
        let _guard = logger::init_logger();

        let rpcurl = std::env::var("RPCURL").expect("RPC_URL not set");
        let pubkey = Signature::from_str("2VTWECcZqLvd1cCazhKfiGvdJTCZU7TxVHnP91UL8AGoB2WEp7USX7hBSr43VAEPesXX8axzwhguSKBdF5AqTGdJ").unwrap();

        let parseclient = DataParse::new(&rpcurl);
        match parseclient{
            Ok(parseclient) => {
                let data = parseclient.signature_to_struct(&pubkey).await.ok();
                if let Some(_data) = data{ 
                    println!("Success");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}