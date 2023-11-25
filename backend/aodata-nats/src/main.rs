#[macro_use]
extern crate dotenv_codegen;

use bytes::Bytes;
use futures_util::stream::StreamExt;
use serde_json::Number;
use sqlx::types::chrono;
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::{Pool, Postgres};
use std::process::exit;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

type Mutex = Arc<RwLock<Vec<Bytes>>>;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let db_url = dotenv!("DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();

    insert_localizations_from_file(dotenv!("LOCALIZATIONS_PATH"), &pool).await.unwrap();
    insert_locations_from_file(dotenv!("LOCATIONS_PATH"), &pool).await.unwrap();

    let mutex: Mutex = Arc::new(RwLock::new(Vec::new()));

    let nats_url = dotenv!("NATS_URL");
    let nats_subject = dotenv!("NATS_SUBJECT");
    let nats_user = dotenv!("NATS_USER");
    let nats_password = dotenv!("NATS_PASSWORD");

    let client = async_nats::ConnectOptions::new()
        .user_and_password(nats_user.to_string(), nats_password.to_string())
        .connect(nats_url)
        .await
        .unwrap();

    print!("Connected to NATS server at {}...\n", nats_url);

    let mut subscriber = client.subscribe(nats_subject).await.unwrap();

    print!("Subscribed to subject {}...\n", nats_subject);

    let join_handle = tokio::spawn(handle_messages(mutex.clone(), pool.clone()));

    while let Some(msg) = subscriber.next().await {
        mutex.write().await.push(msg.payload);
    }

    _ = tokio::join!(join_handle);

    Ok(())
}

async fn handle_messages(mutex: Mutex, pool: Pool<Postgres>) -> Result<(), async_nats::Error> {
    loop {
        let queue_size = mutex.read().await.len();

        if queue_size < 100 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            continue;
        }

        let start = chrono::Utc::now();

        let mut queue_lock = mutex.write().await;
        let queue_slice: Vec<Bytes> = queue_lock.drain(0..queue_size).collect();
        drop(queue_lock);

        let mut messages: Vec<NatsMessage> = Vec::new();

        for message in queue_slice {
            messages.push(serde_json::from_slice(&message).unwrap());
        }

        insert_market_orders(&pool, messages).await.unwrap();

        let end = chrono::Utc::now();

        print!(
            "Inserted {} market orders in {} ms...\n",
            queue_size,
            end.signed_duration_since(start).num_milliseconds()
        );
    }
}

async fn insert_locations_from_file(path: &str, pool: &PgPool) -> Result<(), sqlx::Error> {
    let locations_path = std::path::Path::new(path);

    if !locations_path.exists() {
        print!("Locations file does not exist.\n");
        exit(1);
    }

    let content = std::fs::read_to_string(locations_path).unwrap();

    let locations: Vec<Location> = serde_json::from_str(&content).unwrap();

    let transaction = pool.begin().await.unwrap();

    for location in locations {
        sqlx::query!(
            "INSERT INTO location (id, name) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            location.id,
            location.name
        )
        .execute(pool)
        .await
        .unwrap();
    }

    transaction.commit().await.unwrap();

    Ok(())
}

async fn insert_market_orders(
    pool: &PgPool,
    market_orders: Vec<NatsMessage>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await.unwrap();

    for market_order in market_orders {
        let now = chrono::Utc::now().naive_utc();

        let result = sqlx::query!(
            "INSERT INTO market_order (
                id, 
                item_unique_name, 
                location_id, 
                quality_level, 
                enchantment_level, 
                unit_price_silver, 
                amount, 
                auction_type, 
                expires_at, 
                created_at, 
                updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
                ON CONFLICT (id) DO UPDATE 
                SET unit_price_silver = $6, 
                amount = $7,
                expires_at = $9,
                updated_at = $11",
            market_order.id.as_i64().unwrap(),
            market_order.item_id,
            format!("{:0>4}", market_order.location_id.to_string()),
            market_order.quality_level.as_i64().unwrap() as i32,
            market_order.enchantment_level.as_i64().unwrap() as i32,
            market_order.unit_price_silver.as_i64().unwrap() as i32,
            market_order.amount.as_i64().unwrap() as i32,
            market_order.auction_type,
            chrono::NaiveDateTime::from_str(&market_order.expires.as_str()).unwrap(),
            now,
            now
        )
        .execute(pool)
        .await;

        if result.is_err() {
            print!("Error inserting market order: {:?}\n", result);
        }
    }

    transaction.commit().await.unwrap();

    Ok(())
}

async fn insert_localizations_from_file(path: &str, pool: &PgPool) -> Result<(), sqlx::Error> {

    let localizations_path = std::path::Path::new(path);

    if !localizations_path.exists() {
        print!("Localizations file does not exist.\n");
        exit(1);
    }

    let content = std::fs::read_to_string(localizations_path).unwrap();

    let localizations: Vec<Localization> = serde_json::from_str(&content).unwrap();


    let transaction = pool.begin().await.unwrap();

    for localization in localizations {
        sqlx::query!(
            "INSERT INTO item (unique_name) VALUES ($1) ON CONFLICT DO NOTHING",
            localization.item
        )
        .execute(pool)
        .await
        .unwrap();

        if localization.localized_names.is_some() {
            let localized_names = localization.localized_names.unwrap();

            sqlx::query!(
                "INSERT INTO localized_name (
                    item_unique_name, 
                    en_us, 
                    de_de, 
                    fr_fr, 
                    ru_ru, 
                    pl_pl, 
                    es_es, 
                    pt_br, 
                    it_it, 
                    zh_ch, 
                    ko_kr, 
                    ja_jp, 
                    zh_tw, 
                    id_id) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) ON CONFLICT DO NOTHING",
                localization.item,
                localized_names.en_us,
                localized_names.fr_fr,
                localized_names.ru_ru,
                localized_names.de_de,
                localized_names.pl_pl,
                localized_names.es_es,
                localized_names.pt_br,
                localized_names.it_it,
                localized_names.zh_cn,
                localized_names.ko_kr,
                localized_names.ja_jp,
                localized_names.zh_tw,
                localized_names.id_id,
            )
            .execute(pool)
            .await
            .unwrap();
        }
        if localization.localized_descriptions.is_some() {
            let localized_descriptions = localization.localized_descriptions.unwrap();

            sqlx::query!(
                "INSERT INTO localized_description (
                    item_unique_name, 
                    en_us, 
                    de_de, 
                    fr_fr, 
                    ru_ru, 
                    pl_pl, 
                    es_es, 
                    pt_br, 
                    it_it, 
                    zh_cn, 
                    ko_kr, 
                    ja_jp, 
                    zh_tw, 
                    id_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) ON CONFLICT DO NOTHING",
                localization.item,
                localized_descriptions.en_us,
                localized_descriptions.de_de,
                localized_descriptions.fr_fr,
                localized_descriptions.ru_ru,
                localized_descriptions.pl_pl,
                localized_descriptions.es_es,
                localized_descriptions.pt_br,
                localized_descriptions.it_it,
                localized_descriptions.zh_cn,
                localized_descriptions.ko_kr,
                localized_descriptions.ja_jp,
                localized_descriptions.zh_tw,
                localized_descriptions.id_id,
            )
            .execute(pool)
            .await
            .unwrap();
        }
    }

    transaction.commit().await.unwrap();

    Ok(())
}

#[derive(serde::Deserialize)]
struct Location {
    #[serde(rename = "Index")]
    id: String,
    #[serde(rename = "UniqueName")]
    name: String,
}

#[derive(serde::Deserialize)]
struct Localization {
    #[serde(rename = "LocalizedNames")]
    localized_names: Option<LocalizedValues>,

    #[serde(rename = "LocalizedDescriptions")]
    localized_descriptions: Option<LocalizedValues>,

    #[serde(rename = "UniqueName")]
    item: String,
}

#[derive(serde::Deserialize)]
struct LocalizedValues {
    #[serde(rename = "EN-US")]
    en_us: String,
    #[serde(rename = "DE-DE")]
    de_de: String,
    #[serde(rename = "FR-FR")]
    fr_fr: String,
    #[serde(rename = "RU-RU")]
    ru_ru: String,
    #[serde(rename = "PL-PL")]
    pl_pl: String,
    #[serde(rename = "ES-ES")]
    es_es: String,
    #[serde(rename = "PT-BR")]
    pt_br: String,
    #[serde(rename = "IT-IT")]
    it_it: String,
    #[serde(rename = "ZH-CN")]
    zh_cn: String,
    #[serde(rename = "KO-KR")]
    ko_kr: String,
    #[serde(rename = "JA-JP")]
    ja_jp: String,
    #[serde(rename = "ZH-TW")]
    zh_tw: String,
    #[serde(rename = "ID-ID")]
    id_id: String,
}

#[derive(serde::Deserialize)]
struct NatsMessage {
    #[serde(rename = "Id")]
    id: Number,

    #[serde(rename = "ItemTypeId")]
    item_id: String,

    #[serde(rename = "ItemGroupTypeId")]
    group_type_id: String,

    #[serde(rename = "LocationId")]
    location_id: Number,

    #[serde(rename = "QualityLevel")]
    quality_level: Number,

    #[serde(rename = "EnchantmentLevel")]
    enchantment_level: Number,

    #[serde(rename = "UnitPriceSilver")]
    unit_price_silver: Number,

    #[serde(rename = "Amount")]
    amount: Number,

    #[serde(rename = "AuctionType")]
    auction_type: String,

    #[serde(rename = "Expires")]
    expires: String,
}
