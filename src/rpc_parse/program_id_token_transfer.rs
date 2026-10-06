
use solana_pubkey::Pubkey;
#[derive(Debug)]
pub struct TokenTransfer{
    // 来源 → 从谁的 Token Account 扣钱
    pub source: Pubkey,
    // 目标 → 加到谁的 Token Account
    pub destination: Pubkey,
    // 授权 → 谁拥有从 source 转出的权限
    pub authority: Pubkey,
    // 数量 → 转多少（链上最小单位） 
    pub amount: u64,

}
#[derive(Debug)]
pub struct TokenTransferChecked {
    pub source: Pubkey,
    // 币种 → 扣的是什么 Token
    pub mint: Pubkey,
    pub destination: Pubkey,
    pub authority: Pubkey,
    pub amount: u64,
    // 精度 → 币种的精度
    pub decimals: u8,
}

impl TokenTransfer{
    pub fn new(
        accounts:&[Pubkey],
        data:&[u8],
    )->anyhow::Result<Self>{

        let amount = u64::from_le_bytes(data[1..9].try_into()?);

        Ok(Self { source: accounts[0], destination: accounts[1], authority:accounts[2], amount })
    }
}

impl TokenTransferChecked{
    pub fn new(
        accounts:&[Pubkey],
        data:&[u8],
    )->anyhow::Result<Self>{

        let amount = u64::from_le_bytes(data[1..9].try_into()?);
        let decimals = data[9];

        Ok(Self {
            source: accounts[0],
            mint: accounts[1],
            destination: accounts[2],
            authority: accounts[3],
            amount,
            decimals,
        })
    }
}