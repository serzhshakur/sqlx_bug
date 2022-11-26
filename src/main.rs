use dotenv::dotenv;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};
use std::{env, str::FromStr, time::Duration};

#[derive(Debug)]
pub struct UserData {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

async fn get_pool() -> anyhow::Result<Pool<Sqlite>> {
    let connection_options = SqliteConnectOptions::from_str(&env::var("DATABASE_URL")?)?
        .journal_mode(SqliteJournalMode::Delete)
        .synchronous(SqliteSynchronous::Full)
        .busy_timeout(Duration::from_secs(30));

    let pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = get_pool().await?;

    let username = "johndoe";

    sqlx::query("INSERT OR REPLACE INTO users VALUES ('some_id', 'John', 'Doe', ?)")
        .bind(&username)
        .execute(&pool)
        .await?;

    let user = sqlx::query_as!(
        UserData,
        "UPDATE users SET first_name = 'Bill' WHERE username = ? RETURNING *",
        username
    )
    .fetch_one(&pool)
    .await?;

    println!("user: {user:?}");

    Ok(())
}
