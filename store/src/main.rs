mod schema;

use std::fmt::{Display, Formatter};
use diesel::sql_types::Bool;
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

    async fn update(&mut self, card: Card) -> Result<Card, sqlx::Error> {
        println!("[update]");

        let value: Card = sqlx::query_as("update card set name = $1 where id = $2 returning id, name;")
            .bind(&card.name)
            .bind(&card.id)
            .fetch_one(&self.pool)
            .await?;
        Ok(value)
    }

    async fn get(&self, id: i32) -> Result<Card, sqlx::Error> {
        println!("[get]");

        let user: Card = sqlx::query_as("select id, name from card where id = $1;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn delete(&self, id: i32) -> Result<i32, sqlx::Error> {
        println!("[delete]");
        let result: u64 = sqlx::query("delete from card where id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        match result {
            0 => {
                Ok(0)
            },
            _ => {
                Ok(id)
            }
        }
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
    let new_card = card_manager.create(new_card).await.unwrap();
    println!("{:?}", new_card);

    let cards = Cards::new(card_manager.all().await.unwrap());

    println!("{}", serde_json::to_string(&cards).unwrap());
    println!("{}", cards);

    let card_inserted: Card = card_manager.get(new_card.id).await.unwrap();
    println!("inserted: {:?}", card_inserted);

    let deleted_id = card_manager.delete(card_inserted.id).await.unwrap();
    println!("deleted: {}", deleted_id);

    let mut card = card_manager.get(1).await.unwrap();
    card.name = format!("{} {}", card.name, card.name).into();
    let card_updated = card_manager.update(card).await.unwrap();
    println!("updated: {:?}", card_updated)

}
