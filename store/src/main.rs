mod schema;

use tokio;
use sqlx::{Error, FromRow, Postgres, postgres::{PgPoolOptions}};

#[derive(Debug, FromRow)]
struct Card {
    id: i32,
    name: String,
}

struct CardManager {
    pool: sqlx::Pool<Postgres>,
}

impl CardManager {
    async fn all(&mut self) -> Result<Vec<Card>, sqlx::Error> {
        println!("[all]");

        let value: Vec<Card> = sqlx::query_as("select * from card;")
            .fetch_all(&self.pool)
            .await?;

        Ok(value)
    }
}

#[tokio::main]
async fn main() {
    let pool: Result<sqlx::Pool<Postgres>, Error> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@localhost/hoge").await;
    println!("{:?}", pool);

    let pool: sqlx::Pool<Postgres> = pool.unwrap();
    let mut card_manager = CardManager { pool };

    let cards = card_manager.all().await.unwrap();
    cards.iter().for_each(|c| {
        println!("Hello, {:?}", c);
    })
}
