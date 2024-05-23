use tokio_postgres::{NoTls, Error};
use teloxide::types::{UserId};
use std::env;


static TRANSACTION_TABLE_NAME: &'static str = "transactions";
static LOCATION_TABLE_NAME: &'static str = "users_location_table";


fn get_database_url() -> Result<String, Error> {
    let db_password = env::var("DB_PASSWORD").unwrap_or("".to_string());
    let db_ip_address = env::var("DB_IP_ADDRESS").unwrap_or("".to_string());
    let db_port = env::var("DB_PORT").unwrap_or("".to_string());
    let db_username = env::var("DB_USERNAME").unwrap_or("".to_string());
    let db_name = env::var("DB_NAME").unwrap_or("".to_string());

    let url = format!("postgresql://{}:{}@{}:{}/{}",
                      db_username,
                      db_password,
                      db_ip_address,
                      db_port,
                      db_name
    );
    Ok(url)
}


pub async fn save_transaction(
    user_id: UserId,
    transaction_value: f32,
    latitude: f64,
    longitude: f64,
) -> Result<(), Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    let table_creation_query = format!("CREATE TABLE IF NOT EXISTS {} (
        id SERIAL PRIMARY KEY,
        user_id TEXT,
        transaction_datetime TIMESTAMPTZ DEFAULT NOW(),
        transaction_value REAL,
        transaction_category TEXT,
        latitude DOUBLE PRECISION,
        longitude DOUBLE PRECISION,
        comment TEXT )", TRANSACTION_TABLE_NAME);

    client.execute(&table_creation_query, &[]).await?;

    let insert_query = format!(
        "INSERT INTO {} (user_id, transaction_value, transaction_category, latitude, longitude, comment)\
         VALUES ($1, $2, $3, $4, $5, $6)",
        TRANSACTION_TABLE_NAME);
    client.execute(&insert_query,
                   &[&user_id.to_string(), &transaction_value, &"category",&latitude, &longitude, &"comment"]).await?;

    Ok(())
}


pub async fn get_user_location(user_id: UserId) -> Result<Option<(f64, f64)>, Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });
    let user_location_query = format!("SELECT latitude, longitude FROM {} WHERE user_id = $1",
                               LOCATION_TABLE_NAME);
    if let Ok(Some(row)) = client.query_opt(&user_location_query,
                                    &[&user_id.to_string()],
    ).await {
        let latitude: f64 = row.get(0);
        let longitude: f64 = row.get(1);
        Ok(Some((latitude, longitude)))

    } else {
        Ok(None)

    }
}


pub async fn save_user_location(user_id: UserId, latitude: f64, longitude: f64) -> Result<(), Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });

    let table_creation_query = format!("CREATE TABLE IF NOT EXISTS {} (
        id SERIAL PRIMARY KEY,
        user_id TEXT,
        set_datetime TIMESTAMPTZ DEFAULT NOW(),
        latitude DOUBLE PRECISION,
        longitude DOUBLE PRECISION
        )", LOCATION_TABLE_NAME);

    client.execute(&table_creation_query, &[]).await?;

    let insert_query = format!("INSERT INTO {} (user_id, latitude, longitude)\
                                       VALUES ($1, $2, $3)",
                               LOCATION_TABLE_NAME);
    client.execute(&insert_query, &[&user_id.to_string(), &latitude, &longitude]).await?;

    Ok(())
}