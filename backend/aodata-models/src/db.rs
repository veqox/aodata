use std::str::FromStr;

use super::nats;
use sqlx;

#[derive(sqlx::FromRow, Debug)]
pub struct MarketOrder {
    pub id: i64,
    pub item_unique_name: String,
    pub location_id: String,
    pub quality_level: i32,
    pub enchantment_level: i32,
    pub unit_price_silver: i32,
    pub amount: i32,
    pub auction_type: String,
    pub expires_at: sqlx::types::chrono::NaiveDateTime,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

impl MarketOrder {
    #[allow(dead_code)]
    pub fn from_nats(nats_market_order: &nats::MarketOrder) -> Option<MarketOrder> {
        let expires_at =
            sqlx::types::chrono::NaiveDateTime::from_str(nats_market_order.expires.as_str());

        let expires_at = match expires_at {
            Ok(expires_at) => expires_at,
            Err(_) => return None,
        };

        Some(Self {
            id: nats_market_order.id.as_i64()?,
            item_unique_name: nats_market_order.item_id.clone(),
            location_id: format!("{:0>4}", nats_market_order.location_id),
            quality_level: nats_market_order.quality_level.as_i64()? as i32,
            enchantment_level: nats_market_order.enchantment_level.as_i64()? as i32,
            unit_price_silver: nats_market_order.unit_price_silver.as_i64()? as i32,
            amount: nats_market_order.amount.as_i64()? as i32,
            auction_type: nats_market_order.auction_type.clone(),
            expires_at,
            created_at: sqlx::types::chrono::Utc::now().naive_utc(),
            updated_at: sqlx::types::chrono::Utc::now().naive_utc(),
        })
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct MarketHistory {
    pub item_id: String,
    pub location_id: String,
    pub quality_level: i32,
    pub timescale: i32,
    pub timestamp: sqlx::types::chrono::NaiveDateTime,
    pub item_amount: i32,
    pub silver_amount: i32,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

impl MarketHistory {
    #[allow(dead_code)]
    pub fn from_nats(nats_market_history: nats::MarketHistories) -> Option<Vec<MarketHistory>> {
        let mut market_historie_entries: Vec<MarketHistory> = Vec::new();

        for market_history in nats_market_history.market_histories {
            market_historie_entries.push(Self {
                item_id: nats_market_history.item_id.clone(),
                location_id: format!("{:0>4}", nats_market_history.location_id.to_string()),
                quality_level: nats_market_history.quality_level.as_i64()? as i32,
                timescale: nats_market_history.timescale.as_i64()? as i32,
                timestamp: sqlx::types::chrono::NaiveDateTime::from_timestamp_millis(market_history.timestamp.as_i64()? / 10000 - 62136892800000)?,
                item_amount: market_history.item_amount.as_i64()? as i32,
                silver_amount: market_history.silver_amount.as_i64()? as i32,
                created_at: sqlx::types::chrono::Utc::now().naive_utc(),
                updated_at: sqlx::types::chrono::Utc::now().naive_utc(),
            });
        }

        Some(market_historie_entries)
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Item {
    pub id: String,
    pub unique_name: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct LocalizedName {
    pub item_unique_name: String,
    pub en_us: Option<String>,
    pub de_de: Option<String>,
    pub fr_fr: Option<String>,
    pub ru_ru: Option<String>,
    pub pl_pl: Option<String>,
    pub es_es: Option<String>,
    pub pt_br: Option<String>,
    pub it_it: Option<String>,
    pub zh_cn: Option<String>,
    pub ko_kr: Option<String>,
    pub ja_jp: Option<String>,
    pub zh_tw: Option<String>,
    pub id_id: Option<String>,
    pub tr_tr: Option<String>,
    pub ar_sa: Option<String>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct LocalizedDescription {
    pub item_unique_name: String,
    pub en_us: Option<String>,
    pub de_de: Option<String>,
    pub fr_fr: Option<String>,
    pub ru_ru: Option<String>,
    pub pl_pl: Option<String>,
    pub es_es: Option<String>,
    pub pt_br: Option<String>,
    pub it_it: Option<String>,
    pub zh_cn: Option<String>,
    pub ko_kr: Option<String>,
    pub ja_jp: Option<String>,
    pub zh_tw: Option<String>,
    pub id_id: Option<String>,
    pub tr_tr: Option<String>,
    pub ar_sa: Option<String>,
}
