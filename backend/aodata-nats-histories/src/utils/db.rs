use crate::models::{db, nats};
use sqlx::{postgres::PgQueryResult, types::chrono, PgPool};

pub async fn insert_market_histories(
    pool: &PgPool,
    market_histories: Vec<nats::MarketHistories>,
) -> Result<PgQueryResult, sqlx::Error> {
    let mut item_unique_names: Vec<String> = Vec::new();
    let mut location_ids: Vec<String> = Vec::new();
    let mut quality_levels: Vec<i32> = Vec::new();
    let mut timescales: Vec<i32> = Vec::new();
    let mut timestamps: Vec<chrono::NaiveDateTime> = Vec::new();
    let mut item_amounts: Vec<i32> = Vec::new();
    let mut silver_amounts: Vec<i32> = Vec::new();
    let mut created_ats: Vec<chrono::NaiveDateTime> = Vec::new();
    let mut updated_ats: Vec<chrono::NaiveDateTime> = Vec::new();

    for market_history in market_histories {
        db::MarketHistory::from_nats(market_history)
            .iter()
            .rev()
            .for_each(|market_history| {
                if item_unique_names.contains(&market_history.item_id)
                    && location_ids.contains(&market_history.location_id)
                    && quality_levels.contains(&market_history.quality_level)
                    && timescales.contains(&market_history.timescale)
                    && timestamps.contains(&market_history.timestamp)
                {
                    return;
                }

                item_unique_names.push(market_history.item_id.clone());
                location_ids.push(market_history.location_id.clone());
                quality_levels.push(market_history.quality_level);
                timescales.push(market_history.timescale);
                timestamps.push(market_history.timestamp);
                item_amounts.push(market_history.item_amount);
                silver_amounts.push(market_history.silver_amount);
                created_ats.push(market_history.created_at);
                updated_ats.push(market_history.updated_at);
            });
    }

    let transaction = pool.begin().await.unwrap();

    let result = sqlx::query!(
        "
INSERT INTO market_history (
    item_unique_name,
    location_id,
    quality_level,
    timescale,
    timestamp,
    item_amount,
    silver_amount,
    created_at,
    updated_at)
SELECT * FROM UNNEST(
    $1::VARCHAR[],
    $2::VARCHAR[],
    $3::INT[],
    $4::INT[],
    $5::TIMESTAMP[],
    $6::INT[],
    $7::INT[],
    $8::TIMESTAMP[],
    $9::TIMESTAMP[])
ON CONFLICT (item_unique_name, location_id, quality_level, timescale, timestamp) DO
    UPDATE SET
        updated_at = EXCLUDED.updated_at,
        silver_amount = EXCLUDED.silver_amount,
        item_amount = EXCLUDED.item_amount
        ",
        &item_unique_names,
        &location_ids,
        &quality_levels,
        &timescales,
        &timestamps,
        &item_amounts,
        &silver_amounts,
        &created_ats,
        &updated_ats,
    )
    .execute(pool)
    .await;

    match result {
        Ok(result) => {
            transaction.commit().await.unwrap();
            return Ok(result);
        }
        Err(e) => {
            transaction.rollback().await.unwrap();
            return Err(e);
        }
    }
}
