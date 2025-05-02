use dotenv::dotenv;
use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DB_URL").expect("DB_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let users = sqlx::query(
        r#"
SELECT name,id FROM users;
"#,
    )
    .fetch_all(&pool)
    .await?;

    println!("All users:");

    for user in users {
        println!(
            "ID: {} , NAME: {}",
            user.get::<i32, _>("id"),
            user.get::<&str, _>("name")
        )
    }

    Ok(())
}
