use std::str::FromStr;

use crate::models::{json, nats};
use sqlx::{types::chrono, PgPool};

pub async fn insert_locations(
    pool: &PgPool,
    locations: Vec<json::Location>,
) -> Result<(), sqlx::Error> {
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
    let mut ids: Vec<i64> = Vec::new();
    let mut item_unique_names: Vec<String> = Vec::new();
    let mut location_ids: Vec<String> = Vec::new();
    let mut quality_levels: Vec<i32> = Vec::new();
    let mut enchantment_levels: Vec<i32> = Vec::new();
    let mut unit_price_silvers: Vec<i32> = Vec::new();
    let mut amounts: Vec<i32> = Vec::new();
    let mut auction_types: Vec<String> = Vec::new();
    let mut expires_ats: Vec<chrono::NaiveDateTime> = Vec::new();
    let mut created_ats: Vec<chrono::NaiveDateTime> = Vec::new();
    let mut updated_ats: Vec<chrono::NaiveDateTime> = Vec::new();

    market_orders.iter().rev().for_each(|market_order| {
        if ids.contains(&market_order.id.as_i64().unwrap()) {
            return;
        }

        ids.push(market_order.id.as_i64().unwrap());
        item_unique_names.push(market_order.item_id.clone());
        location_ids.push(format!("{:0>4}", market_order.location_id.to_string()));
        quality_levels.push(market_order.quality_level.as_i64().unwrap() as i32);
        enchantment_levels.push(market_order.enchantment_level.as_i64().unwrap() as i32);
        unit_price_silvers.push(market_order.unit_price_silver.as_i64().unwrap() as i32);
        amounts.push(market_order.amount.as_i64().unwrap() as i32);
        auction_types.push(market_order.auction_type.clone());
        expires_ats.push(chrono::NaiveDateTime::from_str(&market_order.expires.as_str()).unwrap());
        created_ats.push(chrono::Utc::now().naive_utc());
        updated_ats.push(chrono::Utc::now().naive_utc());
    });

    let transaction = pool.begin().await.unwrap();

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
        SELECT * FROM UNNEST(
            $1::BIGINT[], 
            $2::VARCHAR[], 
            $3::VARCHAR[],
            $4::INT[], 
            $5::INT[], 
            $6::INT[], 
            $7::INT[], 
            $8::VARCHAR[], 
            $9::TIMESTAMP[], 
            $10::TIMESTAMP[], 
            $11::TIMESTAMP[]) 
        ON CONFLICT (id) DO 
            UPDATE SET 
                unit_price_silver = EXCLUDED.unit_price_silver, 
                amount = EXCLUDED.amount,
                expires_at = EXCLUDED.expires_at,
                updated_at = EXCLUDED.updated_at",
        &ids,
        &item_unique_names,
        &location_ids,
        &quality_levels,
        &enchantment_levels,
        &unit_price_silvers,
        &amounts,
        &auction_types,
        &expires_ats,
        &created_ats,
        &updated_ats
    )
    .execute(pool)
    .await;

    if result.is_err() {
        print!("{} Error inserting market orders\n", chrono::Local::now());
    }

    match result {
        Ok(_) => {
            transaction.commit().await.unwrap();
        }
        Err(e) => {
            print!(
                "{} Error inserting market orders: {}\n",
                chrono::Local::now(),
                e
            );
            transaction.rollback().await.unwrap();
        }
    }

    Ok(())
}

pub async fn insert_localizations(
    pool: &PgPool,
    localizations: Vec<json::Localization>,
) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await.unwrap();

    for localization in localizations {
        sqlx::query!(
            "INSERT INTO item (unique_name) 
            VALUES ($1) 
            ON CONFLICT DO 
                NOTHING",
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
                VALUES (
                        $1, 
                        $2, 
                        $3, 
                        $4, 
                        $5, 
                        $6, 
                        $7, 
                        $8, 
                        $9, 
                        $10,
                        $11,
                        $12, 
                        $13, 
                        $14) 
                ON CONFLICT DO 
                    NOTHING",
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
                VALUES ( 
                    $1, 
                    $2, 
                    $3, 
                    $4, 
                    $5, 
                    $6, 
                    $7, 
                    $8, 
                    $9, 
                    $10, 
                    $11, 
                    $12, 
                    $13, 
                    $14) 
                ON CONFLICT DO 
                    NOTHING",
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
