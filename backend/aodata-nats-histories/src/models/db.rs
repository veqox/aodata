use sqlx::types::chrono;

use super::nats;

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

impl MarketHistory {
    pub fn from_nats(marketistory: nats::MarketHistories) -> Vec<MarketHistory> {
        let mut market_historie_entries: Vec<MarketHistory> = Vec::new();

        marketistory.market_histories.iter().for_each(|market_history_entrie| {
            market_historie_entries.push(MarketHistory {
                item_id: marketistory.item_id.clone(),
                location_id: format!("{:0>4}", marketistory.location_id.to_string()),
                quality_level: marketistory.quality_level.as_i64().unwrap() as i32,
                timescale: marketistory.timescale.as_i64().unwrap() as i32,
                timestamp: chrono::NaiveDateTime::from_timestamp_micros(market_history_entrie.timestamp.as_i64().unwrap()).unwrap(),
                item_amount: market_history_entrie.item_amount.as_i64().unwrap() as i32,
                silver_amount: market_history_entrie.silver_amount.as_i64().unwrap() as i32,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            });
        });

        market_historie_entries
    }
}