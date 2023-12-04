use std::str::FromStr;

use sqlx::{PgPool, types::chrono};
use crate::models::{json, nats};

pub async fn insert_locations(pool: &PgPool, locations: Vec<json::Location>) -> Result<(), sqlx::Error> {
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

pub async fn insert_market_orders(
    pool: &PgPool,
    market_orders: Vec<nats::MarketOrder>,
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
                updated_at) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
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
            print!("{} Error inserting market order {} \n", chrono::Local::now(), market_order.item_id);
        }
    }

    transaction.commit().await.unwrap();

    Ok(())
}

pub async fn insert_localizations(pool: &PgPool, localizations: Vec<json::Localization>) -> Result<(), sqlx::Error> {
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
                    zh_cn, 
                    ko_kr, 
                    ja_jp, 
                    zh_tw, 
                    id_id) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) ON CONFLICT DO NOTHING",
                localization.item,
                localized_names.en_us,
                localized_names.de_de,
                localized_names.fr_fr,
                localized_names.ru_ru,
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
                    id_id) 
                    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) ON CONFLICT DO NOTHING",
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