use std::collections::HashMap;
use reqwest::{Client, Response, Url};
use scraper::{Html, Selector};
use std::{fs};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::fs::{File, ReadDir};
use std::io::prelude::*;
use async_recursion::async_recursion;
use serde::Deserialize;
use serde_qs;

pub mod wixoss {
    use std::fmt::{Display, Formatter};
    use std::thread::current;
    use scraper::{Html, Selector};
    use regex::Regex;

    #[derive(Debug, Clone, PartialEq)]
    pub enum CardType {
        Lrig,
        Arts,
        Signi,
        Spell,
        Resona,
        ArtsCraft,
        ResonaCraft,
        SpellCraft,
        Piece,
        PieceRelay,
        Token,
    }

    impl Display for CardType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let s: &str = match &self {
                CardType::Lrig => "ルリグ",
                CardType::Arts => "アーツ",
                CardType::Signi => "シグニ",
                CardType::Spell => "スペル",
                CardType::Resona => "レゾナ",
                CardType::ArtsCraft => "アーツ(クラフト)",
                CardType::ResonaCraft => "レゾナ(クラフト)",
                CardType::SpellCraft => "スペル(クラフト)",
                CardType::Piece => "ピース",
                CardType::PieceRelay => "ピース(リレー)",
                CardType::Token => "トークン",
                _ => "不明"
            };
            write!(f, "{}", s)
        }
    }

    pub trait WixossCard: Sized {
        fn from_source(source: String) -> Self;
    }

    // impl Display for dyn WixossCard {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "{}, {}",
    //                &self.no,
    //                &self.name,
    //         )
    //     }
    // }

    #[derive(Debug, Clone)]
    enum Format {
        AllStar,
        KeySelection,
        DivaSelection,
    }

    impl Display for Format {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Format::AllStar => write!(f, "all star"),
                Format::KeySelection => write!(f, "key selection"),
                Format::DivaSelection => write!(f, "diva selection")
            }
        }
    }

    #[derive(Debug, Clone)]
    struct OptionString {
        value: Option<String>,
    }

    impl OptionString {
        fn from_string(value: String) -> Self { // Noneの場合はNoneではなく""空文字
            if value == String::from("") {
                Self { value: None }
            } else {
                Self { value: Some(value) }
            }
        }
    }

    impl Display for OptionString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.value {
                Some(v) => write!(f, "{}", v),
                None => write!(f, "")
            }
        }
    }

    struct OptionInteger {
        value: Option<u32>,
    }

    impl OptionInteger {
        fn from(value: Option<u32>) -> Self {
            match value {
                Some(v) => Self { value: Some(v) },
                None => Self { value: None }
            }
        }
    }

    impl Display for OptionInteger {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.value {
                None => write!(f, ""),
                Some(v) => write!(f, "{}", v.to_string())
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Skills {
        value: Vec<String>,
    }

    impl Skills {
        fn from_vec(skills: Vec<String>) -> Self {
            Self { value: skills }
        }
    }

    impl Display for Skills {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value.join("\n"))
        }
    }

    enum AllCard {
        Piece,
        PieceRelay,
    }

    pub struct Card {
        no: String,
        name: String,
        pronounce: String,
        artist: String,
        pub card_type: CardType,
        color: String,
        level: OptionString,
        cost: OptionString,
        limit: OptionString,
        power: OptionString,
        user: OptionString,
        time: OptionString,
        story: OptionString,
        format: Format,
        rarity: String,
        skill: Skills,
    }

    impl Display for Card {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}",
                   self.no,
                   self.name,
                   self.pronounce,
                   self.artist,
                   self.card_type,
                   self.color,
                   self.level,
                   self.cost,
                   self.limit,
                   self.power,
                   self.user,
                   self.time,
                   self.story,
                   self.format,
                   self.rarity,
                   self.skill
            )
        }
    }

    fn element_to_name_and_pronounce(source: String) -> (String, String) {
        let document = Html::parse_document(&source);

        let br_selector = Selector::parse("br").unwrap();

        let span_selector = Selector::parse("span").unwrap();

        let mut name = String::new();
        let mut pronounce = String::new();

        if let Some(br_element) = document.select(&br_selector).next() {
            if let Some(text_node) = br_element.prev_sibling() {
                name = text_node.value().as_text().unwrap().to_string();
            }
        }

        if let Some(span_element) = document.select(&span_selector).next() {
            pronounce = span_element.inner_html();
        }

        let re_head = Regex::new(r"^＜").unwrap();
        let re_tail = Regex::new(r"＞$").unwrap();

        (name, re_tail.replace(&re_head.replace(&pronounce, ""), "").to_string())
    }


    #[derive(Debug)]
    pub struct Piece {
        no: String,
        name: String,
        pronounce: String,
        artist: String,
        card_type: CardType,
        color: String,
        // level: Option<i32>,
        cost: OptionString,
        // limit: Option<String>,
        // power: Option<String>,
        user: OptionString,
        time: OptionString,
        story: OptionString,
        format: Format,
        rarity: String,
        skill: Skills,
    }

    impl Into<Card> for Piece {
        fn into(self) -> Card {
            Card {
                no: self.no.clone(),
                name: self.name.clone(),
                pronounce: self.pronounce.clone(),
                artist: self.artist.clone(),
                card_type: self.card_type.clone(),
                color: self.color.clone(),
                level: OptionString::from_string("".into()),
                cost: self.cost.clone(),
                limit: OptionString::from_string("".into()),
                power: OptionString::from_string("".into()),
                user: self.user.clone(),
                time: self.time.clone(),
                story: self.story.clone(),
                format: self.format.clone(),
                rarity: self.rarity.clone(),
                skill: self.skill.clone(),
            }
        }
    }

    impl WixossCard for Piece {
        fn from_source(source: String) -> Self {
            let document: Html = Html::parse_document(&source);

            let selector_card_num = Selector::parse(".cardNum").unwrap();
            let card_no = match document.select(&selector_card_num).next() {
                Some(card_no) => card_no.inner_html(),
                None => "unknown".into()
            };

            let selector_card_name = Selector::parse(".cardName").unwrap();
            let card_name = match document.select(&selector_card_name).next() {
                Some(card_name) => element_to_name_and_pronounce(card_name.inner_html()),
                None => ("unknown".into(), "unknown".into())
            };

            let selector_rarity = Selector::parse(".cardRarity").unwrap();
            let card_rarity = match document.select(&selector_rarity).next() {
                Some(card_rarity) => card_rarity.inner_html(),
                None => "unknown rarity".into()
            };

            let selector_artist = Selector::parse(".cardImg p span").unwrap();
            let artist = match document.select(&selector_artist).next() {
                Some(artist) => artist.inner_html(),
                None => "unknown artist".into()
            };

            let selector_card_data = Selector::parse(".cardData dd").unwrap();

            let mut card_data: Vec<String> = Vec::new();
            for element in document.select(&selector_card_data) {
                card_data.push(element.inner_html());
            }

            let selector_card_skill = Selector::parse(".cardSkill").unwrap();
            let card_skill: String = match document.select(&selector_card_skill).next() {
                Some(card_skill) => card_skill.inner_html(),
                None => "No skill".into()
            };


            Self {
                no: card_no,
                name: card_name.0,
                pronounce: card_name.1,
                artist,
                card_type: CardType::Piece,
                color: card_data[2].clone(),
                cost: OptionString::from_string(card_data[5].clone()),
                user: OptionString::from_string(card_data[8].clone()),
                time: OptionString::from_string(card_data[9].clone()),
                story: OptionString::from_string(card_data[11].clone().trim().to_string()),
                format: Format::DivaSelection,
                rarity: card_rarity,
                skill: parse_card_skill(card_skill.clone()),
            }
        }
    }

    #[derive(Debug)]
    pub struct PieceRelay {
        no: String,
        name: String,
        pronounce: String,
        artist: String,
        card_type: CardType,
        color: String,
        // level: Option<i32>,
        cost: OptionString,
        // limit: Option<String>,
        // power: Option<String>,
        user: OptionString,
        time: OptionString,
        story: OptionString,
        format: Format,
        rarity: String,
        skill: Skills,
    }

    impl Into<Card> for PieceRelay {
        fn into(self) -> Card {
            Card {
                no: self.no.clone(),
                name: self.name.clone(),
                pronounce: self.pronounce.clone(),
                artist: self.artist.clone(),
                card_type: self.card_type.clone(),
                color: self.color.clone(),
                level: OptionString::from_string("".into()),
                cost: self.cost.clone(),
                limit: OptionString::from_string("".into()),
                power: OptionString::from_string("".into()),
                user: self.user.clone(),
                time: self.time.clone(),
                story: self.story.clone(),
                format: self.format.clone(),
                rarity: self.rarity.clone(),
                skill: self.skill.clone(),
            }
        }
    }

    impl WixossCard for PieceRelay {
        fn from_source(source: String) -> Self {
            let document: Html = Html::parse_document(&source);

            let selector_card_num = Selector::parse(".cardNum").unwrap();
            let card_no = match document.select(&selector_card_num).next() {
                Some(card_no) => card_no.inner_html(),
                None => "unknown".into()
            };

            let selector_card_name = Selector::parse(".cardName").unwrap();
            let card_name = match document.select(&selector_card_name).next() {
                Some(card_name) => element_to_name_and_pronounce(card_name.inner_html()),
                None => ("unknown".into(), "unknown".into())
            };

            let selector_rarity = Selector::parse(".cardRarity").unwrap();
            let card_rarity = match document.select(&selector_rarity).next() {
                Some(card_rarity) => card_rarity.inner_html(),
                None => "unknown rarity".into()
            };

            let selector_artist = Selector::parse(".cardImg p span").unwrap();
            let artist = match document.select(&selector_artist).next() {
                Some(artist) => artist.inner_html(),
                None => "unknown artist".into()
            };

            let selector_card_data = Selector::parse(".cardData dd").unwrap();

            let mut card_data: Vec<String> = Vec::new();
            for element in document.select(&selector_card_data) {
                card_data.push(element.inner_html());
            }

            let selector_card_skill = Selector::parse(".cardSkill").unwrap();
            let card_skill: String = match document.select(&selector_card_skill).next() {
                Some(card_skill) => card_skill.inner_html(),
                None => "No skill".into()
            };

            Self {
                no: card_no,
                name: card_name.0,
                pronounce: card_name.1,
                artist,
                card_type: CardType::PieceRelay,
                color: card_data[2].clone(),
                cost: OptionString::from_string(card_data[5].clone()),
                user: OptionString::from_string(card_data[8].clone()),
                time: OptionString::from_string(card_data[9].clone()),
                story: OptionString::from_string(card_data[11].clone().trim().to_string()),
                format: Format::DivaSelection,
                rarity: card_rarity,
                skill: parse_card_skill(card_skill),
            }
        }
    }

    #[derive(Debug)]
    pub struct Signi {
        no: String,
        name: String,
        pronounce: String,
        artist: String,
        card_type: CardType,
        color: String,
        level: OptionString,
        // cost: OptionString,
        limit: OptionString,
        // リミット消費
        power: OptionString,
        user: OptionString,
        // time: OptionString,
        story: OptionString,
        format: Format,
        rarity: String,
        skill: Skills,
    }

    impl Into<Card> for Signi {
        fn into(self) -> Card {
            Card {
                no: self.no.clone(),
                name: self.name.clone(),
                pronounce: self.pronounce.clone(),
                artist: self.artist.clone(),
                card_type: self.card_type.clone(),
                color: self.color.clone(),
                level: self.level.clone(),
                cost: OptionString::from_string("".into()),
                limit: self.limit.clone(),
                power: self.power.clone(),
                user: self.user.clone(),
                time: OptionString::from_string("".into()),
                story: self.story.clone(),
                format: self.format.clone(),
                rarity: self.rarity.clone(),
                skill: self.skill.clone(),
            }
        }
    }

    impl WixossCard for Signi {
        fn from_source(source: String) -> Self {
            let document: Html = Html::parse_document(&source);

            let selector_card_num = Selector::parse(".cardNum").unwrap();
            let card_no = match document.select(&selector_card_num).next() {
                Some(card_no) => card_no.inner_html(),
                None => "unknown".into()
            };

            let selector_card_name = Selector::parse(".cardName").unwrap();
            let card_name = match document.select(&selector_card_name).next() {
                Some(card_name) => element_to_name_and_pronounce(card_name.inner_html()),
                None => ("unknown".into(), "unknown".into())
            };

            let selector_rarity = Selector::parse(".cardRarity").unwrap();
            let card_rarity = match document.select(&selector_rarity).next() {
                Some(card_rarity) => card_rarity.inner_html(),
                None => "unknown rarity".into()
            };

            let selector_artist = Selector::parse(".cardImg p span").unwrap();
            let artist = match document.select(&selector_artist).next() {
                Some(artist) => artist.inner_html(),
                None => "unknown artist".into()
            };

            let selector_card_data = Selector::parse(".cardData dd").unwrap();

            let mut card_data: Vec<String> = Vec::new();
            for element in document.select(&selector_card_data) {
                card_data.push(element.inner_html());
            }

            let selector_card_skill = Selector::parse(".cardSkill").unwrap();
            let card_skill: String = match document.select(&selector_card_skill).next() {
                Some(card_skill) => card_skill.inner_html(),
                None => "No skill".into()
            };

            Self {
                no: card_no,
                name: card_name.0,
                pronounce: card_name.1,
                artist,
                card_type: CardType::Signi,
                color: card_data[2].clone(),
                level: OptionString::from_string(card_data[3].clone()),
                // cost: OptionString::from_string(card_data[5].clone()),
                limit: OptionString::from_string(card_data[6].clone()),
                power: OptionString::from_string(card_data[7].clone()),
                user: OptionString::from_string(card_data[8].clone()),
                // time: OptionString::from_string(card_data[9].clone()),
                story: OptionString::from_string(card_data[11].clone().trim().to_string()),
                format: Format::DivaSelection,
                rarity: card_rarity,
                skill: parse_card_skill(card_skill),
            }
        }
    }

    impl Display for Signi {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "NO.\t:{}\n", self.no)?;
            write!(f, "Name\t:{}\n", self.name)?;
            write!(f, "読み\t:{}\n", self.pronounce)?;
            write!(f, "絵\t:{}\n", self.artist)?;
            write!(f, "Type\t:{}\n", self.card_type)?;
            write!(f, "色\t:{}\n", self.color)?;
            write!(f, "レベル\t:{}\n", self.level)?;
            write!(f, "リミット\t:{}\n", self.limit)?;
            write!(f, "パワー\t:{}\n", self.power)?;
            write!(f, "限定\t:{}\n", self.user)?;
            write!(f, "ストーリー\t:{}\n", self.story)?;
            write!(f, "フォーマット\t:{}\n", self.format)?;
            write!(f, "レアリティ\t:{}\n", self.rarity)?;
            write!(f, "テキスト\t:{}\n", self.skill)?;
            write!(f, "")
        }
    }

    fn parse_card_skill(source: String) -> Skills {
        let re_br = Regex::new(r"<br\s?>").unwrap();

        Skills::from_vec(
            re_br
                .replace_all(&source, "\n")
                .split("\n")
                .map(|line| line.trim().to_string())
                .map(remove_rule_explain)
                .filter(|line| !line.is_empty())  // この行を追加して空の行を除去する
                .collect()
        )
    }

    fn remove_rule_explain(text: String) -> String {
        let text = replace_img_with_alt(text);

        let remove_patterns = vec![
            (r"（あなたのルリグの下からカードを合計４枚ルリグトラッシュに置く）", "+EXCEED"),
            (r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに合計１枚までしか入れられない）", "+DREAM TEAM"),
            (r"（あなたの場にいるルリグ３体がこの条件を満たす）", "+TEAM")
        ];
        let replaced_text = remove_patterns.iter().fold(text, |current_text, pat| {
            let re = Regex::new(pat.0).unwrap();
            re.replace_all(&current_text, pat.1).to_string()
        });

        replaced_text
    }

    fn replace_img_with_alt(html: String) -> String {
        let re = Regex::new(r#"<img[^>]*alt="([^"]*)"[^>]*>"#).unwrap();
        let replaced = re.replace_all(&html, |caps: &regex::Captures| {
            let alt_text = &caps[1];
            alt_text.replace("2》", "》")
        });
        replaced.into_owned()
    }
}


#[derive(Clone)]
pub struct SearchQuery {
    search: String,
    keyword: String,
    product_type: ProductType,
    card_page: String,
    card_kind: String,
    rarelity: String,
}

#[derive(Clone)]
pub enum ProductType {
    Booster(String),
    Starter(String),
    PromotionCard,
    SpecialCard,
}

impl ProductType {
    fn get_path_relative(&self) -> String {
        match self {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard => String::from("special"),
        }
    }
}

impl SearchQuery {
    fn new(product_type: &ProductType, card_page: i32) -> SearchQuery {
        SearchQuery {
            search: "".into(),
            keyword: "".into(),
            product_type: product_type.clone(),
            card_page: card_page.to_string(),
            card_kind: "".into(),
            rarelity: "".into(),
        }
    }

    fn get_product_type(&self) -> String {
        match &self.product_type {
            ProductType::Booster(_product_no) => "booster".into(),
            ProductType::Starter(_product_no) => "starter".into(),
            ProductType::PromotionCard => "-".into(),
            ProductType::SpecialCard => "-".into(),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let empty_product_no = String::from("");

        let product_no = match &self.product_type {
            ProductType::Booster(product_no) => product_no,
            ProductType::Starter(product_no) => product_no,
            ProductType::PromotionCard => &empty_product_no,
            ProductType::SpecialCard => &empty_product_no,
        };

        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.get_product_type()),
            ("product_no".into(), product_no.clone()),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
        ])
    }

    fn to_filename(&self) -> String {
        format!("{}/p{}.html", &self.product_type.get_path_relative(), &self.card_page)
    }

    fn cache_check(&self, dir: String) -> Result<String, std::io::Error> {
        let path: PathBuf = PathBuf::from(format!("{}/{}", dir, &self.to_filename()));
        if path.exists() {
            println!("cache found");
            let mut file: File = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            println!("cache not found");
            Err(std::io::Error::new(std::io::ErrorKind::Other, "An unexpected error occurred."))
        }
    }
}

pub fn try_mkdir(rel_path: &Path) -> Result<(), std::io::Error> {
    if !rel_path.exists() {
        fs::create_dir_all(rel_path)?;
    }

    Ok(())
}

#[async_recursion]
pub async fn cache_product_index(product_type: &ProductType, card_page: i32) -> Result<(), reqwest::Error> {
    let p_no = product_type.get_path_relative();
    println!("{} {}", p_no, card_page);

    let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";

    let search_query: SearchQuery = SearchQuery::new(&product_type, card_page);

    let main: Option<String> = match search_query.cache_check("./text_cache".to_string()) {
        Ok(content_) => {
            Some(content_)
        }
        _ => {
            let form: HashMap<String, String> = search_query.to_hashmap();

            let client: Client = Client::new();
            let res: Response = client.post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .form(&form)
                .send().await?;

            let body: String = res.text().await.unwrap();

            let cache_filename: PathBuf = PathBuf::from(format!("./text_cache/{}", &search_query.to_filename()));

            if let Some(parent_path) = cache_filename.parent() {
                try_mkdir(&parent_path).unwrap();

                let content = find_one(&body, ".cardDip".into());

                if let Some(element) = &content {
                    let file: Result<File, std::io::Error> = File::create(&cache_filename);
                    if let Ok(mut file_) = file {
                        file_.write_all(element.as_bytes()).unwrap();
                    }
                }
                content
            } else {
                None
            }
        }
    };

    if let Some(count) = find_one(&main.unwrap(), "h3 p span".into()) {
        let count = extract_number(&count);

        if let Some(count) = count {
            let pages = (count / 21) + 1;

            if card_page < pages {
                cache_product_index(&product_type, card_page + 1).await.unwrap();
            }
        }
    } else {
        println!("not found");
    }


    Ok(())
}

pub fn find_one(content: &String, selector: String) -> Option<String> {
    let document: Html = Html::parse_document(&content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();

    println!("{:?}", document.clone());
    if let Some(element) = document.select(&main_selector).next() {
        Some(element.inner_html())
    } else {
        None
    }
}

pub async fn collect_card_detail_links(product_type: &ProductType) -> Result<Vec<String>, ()> {
    let product_root: String = product_type.get_path_relative();
    let path_s: String = format!("./text_cache/{}", product_root);
    let product_dir: &Path = Path::new(&path_s);

    println!("{}", product_dir.display());

    try_mkdir(&product_dir).unwrap();
    let files_result: std::io::Result<ReadDir> = fs::read_dir(product_dir);

    match files_result {
        Ok(files) => {
            let all_text: String = files.into_iter().map(|f| {
                let p: fs::DirEntry = f.unwrap();
                let file_path: PathBuf = p.path();
                let content: String = fs::read_to_string(&file_path).unwrap();

                content
            }).collect::<Vec<_>>().join("");

            let parsed_html: Html = Html::parse_document(&all_text);
            let selector: Selector = Selector::parse("a.c-box").unwrap();
            let links: Vec<String> = parsed_html.select(&selector).into_iter().map(|element| {
                element.value().attr("href")
                    .unwrap_or("").to_owned()
            }).filter(|href| !href.is_empty()).collect();
            Ok(links)
        }
        Err(err) => {
            println!("{:?}", err);
            Err(())
        }
    }
}

#[allow(dead_code)]
pub fn find_many(content: &String, selector: String) -> Vec<String> {
    let document: Html = Html::parse_document(&content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();
    let mut result: Vec<String> = Vec::new();
    for element in document.select(&main_selector) {
        result.push(element.inner_html());
    }
    result
}


pub fn extract_number(s: &String) -> Option<i32> {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse().ok()
}

#[derive(Debug, Deserialize)]
pub struct CardQuery {
    card: String,
    card_no: String,
}

impl CardQuery {
    pub fn get_relative_filename(&self) -> String {
        let mut tokens: Vec<_> = self.card_no.split("-").collect();
        let id = tokens.last().unwrap().to_string();
        tokens.pop();
        let dir: String = tokens.join("-");

        format!("{}/{}.html", dir, id)
    }

    pub fn from_card_no(card_no: String) -> Self {
        Self {
            card_no,
            card: "card_detail".into(),
        }
    }
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        HashMap::from_iter(vec![
            ("card_no".into(), self.card_no.clone()),
            ("card".into(), self.card.clone()),
        ])
    }

    pub async fn download_card_detail(&self, cache_dir: &'static str) -> Option<String> {
        let cache_file: PathBuf = PathBuf::from(format!("{}/{}", cache_dir, self.get_relative_filename()));

        println!("{:?}", cache_file);
        if cache_file.exists() {
            let mut file: File = File::open(&cache_file).expect("cache file open error");
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => Some(contents),
                _ => { None }
            }
        } else {
            let url = "https://www.takaratomy.co.jp/products/wixoss/card_list.php";

            let form: HashMap<String, String> = self.to_hashmap();

            let client: Client = Client::new();
            let res: Result<Response, reqwest::Error> = client.post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .form(&form)
                .send().await;

            match res {
                Ok(response) => {
                    let body: String = response.text().await.unwrap();
                    let body: String = format!("<html><body>{}", body);
                    let content = find_one(&body, ".cardDetail".into());

                    if let Some(body_) = content {
                        match write_to_cache(cache_file, body_.clone()) {
                            Ok(()) => Some(body_),
                            _ => None
                        }
                    } else {
                        println!("{}", body);
                        None
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        }
    }
}

pub fn parse_card_url(url_string: impl Display) -> Result<CardQuery, serde_qs::Error> {
    let parsed_url: Url = Url::parse(&url_string.to_string()).expect("Failed to parse the URL");
    let query: &str = parsed_url.query().unwrap_or_default();
    serde_qs::from_str::<CardQuery>(query)
}

pub fn write_to_cache(filename: PathBuf, body: String) -> Result<(), ()> {
    if let Some(parent_path) = filename.parent() {
        try_mkdir(&parent_path).unwrap();
        let file: Result<File, std::io::Error> = File::create(&filename);
        if let Ok(mut file_) = file {
            file_.write_all(body.as_bytes()).unwrap();
        }
        Ok(())
    } else {
        Err(())
    }
}

