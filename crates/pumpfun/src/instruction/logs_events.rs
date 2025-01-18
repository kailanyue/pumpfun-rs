use crate::instruction::logs_data::{CreateTokenInfo, TradeInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DexEvent {
    NewToken(CreateTokenInfo),
    NewUserTrade(TradeInfo),
    NewBotTrade(TradeInfo),
    Error(String),
}
