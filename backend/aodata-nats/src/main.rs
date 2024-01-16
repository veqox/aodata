#[macro_use]
extern crate dotenv_codegen;

use bytes::Bytes;
use futures_util::stream::StreamExt;
use sqlx::types::chrono;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;

mod models;
use models::nats;

mod utils;

type Mutex = Arc<RwLock<Vec<Bytes>>>;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let db_url = match std::env::var("ENV") {
        Ok(env) => match env.as_str() {
            "PROD" => dotenv!("PROD_DATABASE_URL"),
            "DEV" => dotenv!("DATABASE_URL"),
            _ => dotenv!("DATABASE_URL"),
        },
        Err(_) => dotenv!("DATABASE_URL"),
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();

    let localizations = utils::json::get_localizations_from_file(dotenv!("LOCALIZATIONS_PATH"));
    let locations = utils::json::get_locations_from_file(dotenv!("LOCATIONS_PATH"));

    if localizations.is_none() || locations.is_none() {
        panic!("Failed to load localizations or locations from file!");
    }

    utils::db::insert_localizations(&pool, localizations.unwrap())
        .await
        .unwrap();
    utils::db::insert_locations(&pool, locations.unwrap())
        .await
        .unwrap();

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

    let message_handler = tokio::spawn(handle_messages(mutex.clone(), pool.clone()));
    let materialized_views_updater = tokio::spawn(update_materialized_views(pool.clone()));

    print!(
        "{} Connected to NATS server at {}...\n",
        chrono::Local::now(),
        nats_url
    );

    let mut subscriber = client.subscribe(nats_subject).await.unwrap();

    print!(
        "{} Subscribed to subject {}...\n",
        chrono::Local::now(),
        nats_subject
    );

    while let Some(msg) = subscriber.next().await {
        mutex.write().await.push(msg.payload);
    }

    _ = tokio::join!(message_handler, materialized_views_updater);

    pool.close().await;

    Ok(())
}

async fn update_materialized_views(pool: Pool<Postgres>) -> Result<(), async_nats::Error> {
    print!(
        "{} update_materialized_views: Starting update materialized view thread...\n",
        chrono::Local::now()
    );

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 10)).await;

        print!(
            "{} update_materialized_views: Updating materialized views...\n",
            chrono::Local::now()
        );

        let transaction = pool.begin().await.unwrap();

        let _ = sqlx::query!("REFRESH MATERIALIZED VIEW market_orders_count_by_created_at_and_location").execute(&pool).await.unwrap();
        let _ = sqlx::query!("REFRESH MATERIALIZED VIEW market_orders_count_by_updated_at_and_location").execute(&pool).await.unwrap();
        let _ = sqlx::query!("REFRESH MATERIALIZED VIEW market_orders_count_by_created_at").execute(&pool).await.unwrap();
        let _ = sqlx::query!("REFRESH MATERIALIZED VIEW market_orders_count_by_updated_at").execute(&pool).await.unwrap();
        let _ = sqlx::query!("REFRESH MATERIALIZED VIEW market_orders_count_by_location").execute(&pool).await.unwrap();

        transaction.commit().await.unwrap();

        print!(
            "{} update_materialized_views: Updated materialized views...\n",
            chrono::Local::now()
        ); 
    }
}

async fn handle_messages(mutex: Mutex, pool: Pool<Postgres>) -> Result<(), async_nats::Error> {
    print!(
        "{} handle_messages: Starting message handler thread...\n",
        chrono::Local::now()
    );

    loop {
        let queue_size = mutex.read().await.len();

        if queue_size < 1000 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            continue;
        }

        let start = chrono::Utc::now();

        let mut queue_lock = mutex.write().await;
        let queue_slice: Vec<Bytes> = queue_lock.drain(0..queue_size).collect();
        drop(queue_lock);

        let mut messages: Vec<nats::MarketOrder> = Vec::new();

        for message in queue_slice {
            messages.push(serde_json::from_slice(&message).unwrap());
        }

        let rows_affected = utils::db::insert_market_orders(&pool, messages).await.unwrap();

        let end = chrono::Utc::now();

        print!(
            "{} handle_messages: Inserted {} market orders in {} ms...\n",
            chrono::Local::now(),
            rows_affected,
            end.signed_duration_since(start).num_milliseconds()
        );
    }
}
