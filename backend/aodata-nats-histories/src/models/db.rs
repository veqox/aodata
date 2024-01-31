use sqlx::types::chrono;

pub struct MarketHistory {
    pub item_id: String,
    pub location_id: String,
    pub quality_level: i32,
    pub timescale: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub item_amount: i32,
    pub silver_amount: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}