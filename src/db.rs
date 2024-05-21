use tokio_postgres::{NoTls, Error};
use std::env;


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

pub async fn save_transaction(transaction_value: f32) -> Result<(), Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });
    let transactions_table_name: &str = "transactions";
    let table_creation_query = format!("CREATE TABLE IF NOT EXISTS {} (
        id SERIAL PRIMARY KEY,
        user_id INTEGER,
        transaction_datetime TIMESTAMPTZ DEFAULT NOW(),
        transaction_value REAL,
        transaction_category TEXT,
        comment TEXT )", transactions_table_name);

    client.execute(&table_creation_query, &[]).await?;

    let insert_query = format!("INSERT INTO {} (user_id, transaction_value, transaction_category, comment) VALUES ($1, $2, $3, $4)",
                               transactions_table_name);
    client.execute(&insert_query, &[&1, &transaction_value, &"category", &"comment"]).await?;

    Ok(())
}