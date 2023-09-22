use scraping_sample::{
    parse_card_url,
    CardQuery,
};

#[tokio::main]
async fn main() {
    let cq: CardQuery = CardQuery::from_card_no("WXDi-P14-001".into());
    let text: Option<String> = cq.download_card_detail("./text_cache").await;
    println!("{}", text.unwrap_or("detail download error".into()))
}