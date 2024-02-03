use serde;
use serde_json;
use bytes;

#[derive(serde::Deserialize, Debug)]
pub struct MarketOrder {
    #[serde(rename = "Id")]
    pub id: serde_json::Number,
    #[serde(rename = "ItemTypeId")]
    pub item_id: String,
    #[serde(rename = "ItemGroupTypeId")]
    pub group_type_id: String,
    #[serde(rename = "LocationId")]
    pub location_id: serde_json::Number,
    #[serde(rename = "QualityLevel")]
    pub quality_level: serde_json::Number,
    #[serde(rename = "EnchantmentLevel")]
    pub enchantment_level: serde_json::Number,
    #[serde(rename = "UnitPriceSilver")]
    pub unit_price_silver: serde_json::Number,
    #[serde(rename = "Amount")]
    pub amount: serde_json::Number,
    #[serde(rename = "AuctionType")]
    pub auction_type: String,
    #[serde(rename = "Expires")]
    pub expires: String,
}

impl MarketOrder {
    #[allow(dead_code)]
    pub fn parse_json(json: bytes::Bytes) -> Result<MarketOrder, serde_json::Error> {
        serde_json::from_slice(&json)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MarketHistories {
    #[serde(rename = "AlbionId")]
    pub id: serde_json::Number,
    #[serde(rename = "AlbionIdString")]
    pub item_id: String,
    #[serde(rename = "LocationId")]
    pub location_id: serde_json::Number,
    #[serde(rename = "QualityLevel")]
    pub quality_level: serde_json::Number,
    #[serde(rename = "Timescale")]
    pub timescale: serde_json::Number,
    #[serde(rename = "MarketHistories")]
    pub market_histories: Vec<MarketHistory>,
}

impl MarketHistories {
    #[allow(dead_code)]
    pub fn parse_json(json: bytes::Bytes) -> Result<MarketHistories, serde_json::Error> {
        serde_json::from_slice(&json)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MarketHistory {
    #[serde(rename = "ItemAmount")]
    pub item_amount: serde_json::Number,
    #[serde(rename = "SilverAmount")]
    pub silver_amount: serde_json::Number,
    #[serde(rename = "Timestamp")]
    pub timestamp: serde_json::Number,
}
