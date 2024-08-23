use tokio_postgres::{NoTls, Error};
use teloxide::types::{UserId};
use std::env;

static TRANSACTION_TABLE_NAME: &'static str = "transactions";
static LOCATION_TABLE_NAME: &'static str = "users_location_table";
static CURRENCY_TABLE_NAME: &'static str = "currency_table";


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
    currency: &str,
    latitude: f64,
    longitude: f64,
    category: &str,
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
        currency TEXT,
        transaction_category TEXT,
        latitude DOUBLE PRECISION,
        longitude DOUBLE PRECISION,
        comment TEXT )", TRANSACTION_TABLE_NAME);

    client.execute(&table_creation_query, &[]).await?;

    let insert_query = format!(
        "INSERT INTO {} (user_id, transaction_value, currency, transaction_category, latitude, longitude, comment)\
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        TRANSACTION_TABLE_NAME);
    client.execute(&insert_query,
                   &[&user_id.to_string(), &transaction_value, &currency, &category,&latitude, &longitude, &"comment"]).await?;

    Ok(())
}


pub async fn get_user_location(
    user_id: UserId,
) -> Result<Option<(f64, f64)>, Error> {
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


pub async fn save_user_location(
    user_id: UserId,
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


pub async fn set_user_currency(
    user_id: UserId,
    currency: &str,
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
        currency TEXT
        )", CURRENCY_TABLE_NAME);

    client.execute(&table_creation_query, &[]).await?;

    if let Ok(current_currency) = get_user_currency(user_id
    ).await {
        if current_currency == "" {
            let insert_query = format!("INSERT INTO {} (user_id, currency)\
                                       VALUES ($1, $2)",
                                       CURRENCY_TABLE_NAME);
            client.execute(&insert_query, &[&user_id.to_string(), &currency]).await?;
        } else {
            let insert_query = format!("UPDATE {} SET currency = $1 WHERE user_id = $2",
                                       CURRENCY_TABLE_NAME);
            client.execute(&insert_query, &[&currency.to_string(), &user_id.to_string()]).await?;
        }
        Ok(())
    } else {
        let insert_query = format!("INSERT INTO {} (user_id, currency)\
                                       VALUES ($1, $2)",
                                   CURRENCY_TABLE_NAME);
        client.execute(&insert_query, &[&user_id.to_string(), &currency]).await?;
        Ok(())
    }
}


pub async fn get_user_currency(
    user_id: UserId,
) -> Result<String, Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });
    let user_currency_query = format!(
        "SELECT currency FROM {} WHERE user_id = $1",
        CURRENCY_TABLE_NAME);

    match client.query_opt(&user_currency_query, &[&user_id.to_string()],
    ).await {
        Ok(Some(row)) => {
            let currency: String = row.get(0);
            Ok(currency)
        }
        Ok(None) => {
            let currency = "".to_string();
            Ok(currency)
        }
        Err(e) => {
            Err(Error::from(e))
        }
    }
}


pub async fn delete_last_transaction(
    user_id: UserId,
) -> Result<Option<String>, Error> {
    let url = get_database_url()?;
    let (client, connection) = tokio_postgres::connect(&url, NoTls).await?;
    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Connection Error: {}", e);
        }
    });
    let last_user_transaction_query = format!(
        "SELECT id, transaction_value, transaction_category FROM {} WHERE user_id = $1 ORDER BY transaction_datetime DESC LIMIT 1",
                                      TRANSACTION_TABLE_NAME);
    if let Ok(Some(row)) = client.query_opt(&last_user_transaction_query,
                                            &[&user_id.to_string()],
    ).await {
        let transaction_id: i32 = row.get(0);
        let transaction_value: f32 = row.get(1);
        let transaction_category: &str = row.get(2);
        let delete_last_user_transaction_query = format!("DELETE FROM {} WHERE id = $1",
                                                         TRANSACTION_TABLE_NAME);
        let transaction_description;
        if let Ok(_) = client.execute(&delete_last_user_transaction_query,&[&transaction_id]).await {
            transaction_description = format!(
                "Транзакция на сумму {} в категории {} была удалена", &transaction_value, transaction_category);
        } else {
            transaction_description = format!(
                "Транзакция на сумму {} в категории {} НЕ была удалена", &transaction_value, transaction_category);
        }
        Ok(Some(transaction_description))
    } else {
        Ok(None)
    }
}

