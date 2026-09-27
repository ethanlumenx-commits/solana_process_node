/// 计算资源预算
/// “这笔交易准备消耗多少计算资源？”
/// “我愿意为计算资源支付多少优先费用？”
/// “程序需要多大的 heap？”
/// “最多加载多少账户数据？”
/// CU = Compute Unit Solana 执行程序时使用的“计算资源单位”
#[derive(Debug)]
pub enum ComputeBudgetInstruction {
    /// 1.设置程序执行时的 heap 内存大小  “我需要多大的临时工作内存？”
    RequestHeapFrame { bytes: u32 },
    /// 2.设置程序执行时的计算资源预算  “我需要多少计算资源？”
    SetComputeUnitLimit { units: u32 },
    /// 3.设置每个 CU 愿意支付多少 micro-lamports  “我愿意为计算资源付多少钱，以提高优先级。”
    SetComputeUnitPrice { micro_lamports: u64 },
    /// 4.限制交易执行时可以加载的 账户数据总量	 “这次最多允许加载这么多账户数据。”
    SetLoadedAccountsDataSizeLimit { bytes: u32 },
}

impl ComputeBudgetInstruction {
    pub fn new(data:&[u8])->anyhow::Result<Self>{
        match data[0] { 
            1 =>{
                if data.len() < 5{ 
                    return Err(anyhow::anyhow!("Invalid ComputeBudgetInstruction data length{}",data.len()));
                }
                let bytes = u32::from_le_bytes(data[1..5].try_into()?);
                Ok(Self::RequestHeapFrame { bytes })
            }
            2 =>{
                if data.len() < 5{ 
                    return Err(anyhow::anyhow!("Invalid ComputeBudgetInstruction data length{}",data.len()));
                }
                let units = u32::from_le_bytes(data[1..5].try_into()?);
                Ok(Self::SetComputeUnitLimit { units })
            }
            3 =>{
                if data.len() < 9{ 
                    return Err(anyhow::anyhow!("Invalid ComputeBudgetInstruction data length{}",data.len()));
                }
                let micro_lamports = u64::from_le_bytes(data[1..9].try_into()?);
                Ok(Self::SetComputeUnitPrice { micro_lamports })
            }
            4 =>{
                if data.len() < 5{ 
                    return Err(anyhow::anyhow!("Invalid ComputeBudgetInstruction data length{}",data.len()));
                }
                let bytes = u32::from_le_bytes(data[1..5].try_into()?);
                Ok(Self::SetLoadedAccountsDataSizeLimit { bytes })
            }
            _=>{
                Err(anyhow::anyhow!("Invalid ComputeBudgetInstruction"))
            }
        }

    }
}