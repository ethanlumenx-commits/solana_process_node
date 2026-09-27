
use solana_pubkey::Pubkey;
#[derive(Debug)]
/// System Program → 转 SOL
/// Token Program  → 转 Token
/// Jupiter         → Swap
pub struct SystemTransfer{
    pub from: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
}

impl SystemTransfer{
    pub fn new(
        accounts:&[Pubkey],
        data:&[u8],
    )->anyhow::Result<Self>{

        let lamports  = u64::from_le_bytes(data[4..12].try_into()?);

        if accounts.len() != 2{
            return Err(anyhow::anyhow!("Invalid accounts length {}",accounts.len()));
        }

        Ok(Self{
            from: accounts[0],
            to: accounts[1],
            lamports: lamports ,
        })
    }
}