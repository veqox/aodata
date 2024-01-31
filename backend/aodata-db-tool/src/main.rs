#[macro_use]
extern crate dotenv_codegen;

mod utils;
mod models;

#[tokio::main]
async fn main() {
    let db_url = dotenv!("DATABASE_URL");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap();
    
    let localizations = utils::json::get_localizations_from_file(dotenv!("LOCALIZATIONS_PATH"));
    let locations = utils::json::get_locations_from_file(dotenv!("LOCATIONS_PATH"));

    let localizations = match localizations {
        Some(localizations) => localizations,
        None => panic!("Error getting localizations from file"),
    };

    let locations = match locations {
        Some(locations) => locations,
        None => panic!("Error getting locations from file"),
    };

    let result = utils::db::insert_localizations(&pool, localizations).await;

    match result {
        Ok(_) => println!("Inserted localizations"),
        Err(e) => println!("Error inserting localizations: {}", e),
    }
    
    let result = utils::db::insert_locations(&pool, locations).await;

    match result {
        Ok(_) => println!("Inserted locations"),
        Err(e) => println!("Error inserting locations: {}", e),
    }

    println!("Done!");

    pool.close().await;
}
