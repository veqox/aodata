#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Item {
    pub unique_name: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct LocalizedName {
    pub item_unique_name: String,
    pub en_us: String,
    pub de_de: String,
    pub fr_fr: String,
    pub ru_ru: String,
    pub pl_pl: String,
    pub es_es: String,
    pub pt_br: String,
    pub it_it: String,
    pub zh_cn: String,
    pub ko_kr: String,
    pub ja_jp: String,
    pub zh_tw: String,
    pub id_id: String,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct LocalizedDescription {
    pub item_unique_name: String,
    pub en_us: String,
    pub de_de: String,
    pub fr_fr: String,
    pub ru_ru: String,
    pub pl_pl: String,
    pub es_es: String,
    pub pt_br: String,
    pub it_it: String,
    pub zh_cn: String,
    pub ko_kr: String,
    pub ja_jp: String,
    pub zh_tw: String,
    pub id_id: String,
}

#[derive(serde::Serialize)]
pub struct Localizations {
    pub unique_name: String,
    pub name: LocalizedName,
    pub description: LocalizedDescription,
}

#[derive(serde::Serialize)]
pub struct LocalizationResponse {
    pub unique_name: String,
    pub name: Option<LocalizedName>,
    pub description: Option<LocalizedDescription>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrder {
    pub id: i64,
    pub item_unique_name: String,
    pub location_id: String,
    pub quality_level: i32,
    pub enchantment_level: i32,
    pub unit_price_silver: i32,
    pub amount: i32,
    pub auction_type: String,
    pub expires_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCount {
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByItem {
    pub item_unique_name: String,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByLocation {
    pub location: Option<String>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByAuctionType {
    pub auction_type: String,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByQualityLevel {
    pub quality_level: i32,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByEnchantmentLevel {
    pub enchantment_level: i32,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByUpdatedAtAndLocation {
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByCreatedAtAndLocation {
    pub created_at: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByUpdatedAt {
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct MarketOrderCountByCreatedAt {
    pub created_at: Option<chrono::NaiveDateTime>,
    pub count: Option<i64>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct SearchResult {
    pub item_unique_name: String,
    pub en_us: String,
    pub de_de: String,
    pub fr_fr: String,
    pub ru_ru: String,
    pub pl_pl: String,
    pub es_es: String,
    pub pt_br: String,
    pub it_it: String,
    pub zh_cn: String,
    pub ko_kr: String,
    pub ja_jp: String,
    pub zh_tw: String,
    pub id_id: String,
    pub rank: Option<f32>,
}