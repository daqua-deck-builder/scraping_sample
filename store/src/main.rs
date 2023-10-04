mod schema;

use std::fmt::{Display, Formatter};
use tokio;
use sqlx::{ColumnIndex, Error, FromRow, Postgres, postgres::{PgPoolOptions}};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, FromRow, Serialize)]
struct Card {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct Cards {
    cards: Vec < Card>
}

impl Cards {
    fn new(cards: Vec<Card>) -> Self {
        Self {
            cards
        }
    }
}

impl Display for Cards {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let s: &String = &self.cards.iter().map(|c| {
            String::from(format!("{} {}\n", c.id, c.name))
        }).collect();
        write!(f, "{}", s).unwrap();
        Ok(())
    }
}

#[derive(Deserialize)]
struct NewCard {
    name: String
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

    async fn create(&mut self, nc: NewCard) -> Result<Card, sqlx::Error> {
        println!("[create]");

        let value: Card = sqlx::query_as("insert into card (name) values ($1) returning id, name;")
            .bind(&nc.name)
            .fetch_one(&self.pool)
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

    let new_card: NewCard = NewCard {name: "BBB".into()};
    let new_card = card_manager.create(new_card).await;
    println!("{:?}", new_card.unwrap());

    let cards = Cards::new(card_manager.all().await.unwrap());

    println!("{}", serde_json::to_string(&cards).unwrap());
    println!("{}", cards)
}
