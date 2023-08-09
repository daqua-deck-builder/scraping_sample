use scraping_sample::scrape::{
    parse_card_url,
    CardQuery,
};

#[tokio::main]
async fn main() {
    let cq: CardQuery = parse_card_url("https://www.takaratomy.co.jp/products/wixoss/card_list.php?card=card_detail&card_no=WXDi-P12-090").expect("query string parse error");
    println!("{:?}", cq);
}