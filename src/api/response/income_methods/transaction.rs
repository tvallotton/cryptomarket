use crate::api::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub r#type: String,
    pub subtype: String,
    pub native: Native,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Native {
    pub tx_id: String,
    pub index: i64,
    pub currency: String,
    pub amount: String,
    pub fee: String,
    pub hash: String,
    pub address: String,
    pub confirmations: i64,
}
