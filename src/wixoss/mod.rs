mod constants;

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use scraper::{Html, Selector};
use regex::Regex;
use crate::wixoss::constants::CardFeature;

#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    Lrig,
    LrigAssist,
    Arts,
    Key,
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
            CardType::LrigAssist => "ルリグ(アシスト)",
            CardType::Arts => "アーツ",
            CardType::Key => "キー",
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

#[derive(Debug, Clone, PartialEq)]
pub struct OptionString {
    value: Option<String>,
}

impl OptionString {
    pub fn from_string(value: String) -> Self { // Noneの場合はNoneではなく""空文字
        if value == String::from("") {
            Self { value: None }
        } else {
            Self { value: Some(value) }
        }
    }

    pub fn empty() -> Self {
        Self { value: None }
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
    klass: OptionString,
    color: String,
    level: OptionString,
    cost: OptionString,
    limit: OptionString,
    power: OptionString,
    user: OptionString,
    time: Vec<String>,
    pub story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}\n{}",
               self.no,
               self.name,
               self.pronounce,
               self.artist,
               self.card_type,
               self.klass,
               self.color,
               self.level,
               self.cost,
               self.limit,
               self.power,
               self.user,
               self.time.join(", "),
               self.story,
               self.format,
               self.rarity,
               self.skill,
               self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")
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
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for Piece {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: OptionString::empty(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Piece,
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[8].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        // write!(f, "種族\t:{}\n", self.klass)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "限定\t:{}\n", self.user)?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
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
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for PieceRelay {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: OptionString::empty(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),

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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::PieceRelay,
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[8].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for PieceRelay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        // write!(f, "種族\t:{}\n", self.klass)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "限定\t:{}\n", self.user)?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct Signi {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    klass: OptionString,
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
    features: HashSet<CardFeature>,
}

impl Into<Card> for Signi {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: self.klass.clone(),
            color: self.color.clone(),
            level: self.level.clone(),
            cost: OptionString::from_string("".into()),
            limit: self.limit.clone(),
            power: self.power.clone(),
            user: self.user.clone(),
            time: Vec::new(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Signi,
            klass: OptionString::from_string(card_data[1].clone()),
            color: card_data[2].clone(),
            level: OptionString::from_string(card_data[3].clone()),
            // cost: OptionString::from_string(card_data[5].clone()),
            limit: OptionString::from_string(card_data[6].clone()),
            power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[8].clone()),
            // time: OptionString::from_string(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
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
        write!(f, "種族\t:{}\n", self.klass)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "リミット\t:{}\n", self.limit)?;
        write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "限定\t:{}\n", self.user)?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct Spell {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    color: String,
    // level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    // power: OptionString,
    user: OptionString,
    // time: OptionString,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for Spell {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: OptionString::empty(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: Vec::new(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for Spell {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Spell,
            color: card_data[2].clone(),
            // level: OptionString::from_string(card_data[3].clone()),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            // limit: OptionString::from_string(card_data[6].clone()),
            // power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[8].clone()),
            // time: OptionString::from_string(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for Spell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        // write!(f, "専用上限\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "限定\t:{}\n", self.user)?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct Lrig {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: String,
    level: OptionString,
    cost: OptionString,
    limit: OptionString,
    // power: OptionString,
    user: OptionString,
    // time: OptionString,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for Lrig {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: self.level.clone(),
            cost: self.cost.clone(),
            limit: self.limit.clone(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: Vec::new(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for Lrig {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Lrig,
            color: card_data[2].clone(),
            level: OptionString::from_string(card_data[3].clone()),
            cost: OptionString::from_string(flatten_break(card_data[4].clone())),
            limit: OptionString::from_string(card_data[6].clone()),
            // power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[1].clone()),
            // time: OptionString::from_string(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for Lrig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "グロウコスト\t:{}\n", self.cost)?;
        write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct LrigAssist {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: String,
    level: OptionString,
    cost: OptionString,
    limit: OptionString,
    // power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for LrigAssist {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: self.level.clone(),
            cost: self.cost.clone(),
            limit: self.limit.clone(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for LrigAssist {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::LrigAssist,
            color: card_data[2].clone(),
            level: OptionString::from_string(card_data[3].clone()),
            cost: OptionString::from_string(flatten_break(card_data[4].clone())),
            limit: OptionString::from_string(card_data[6].clone()),
            // power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[1].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for LrigAssist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "グロウコスト\t:{}\n", self.cost)?;
        write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct Arts {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: String,
    // level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    // power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for Arts {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: OptionString::empty(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for Arts {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Arts,
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[1].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for Arts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Resona {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    klass: OptionString,
    color: String,
    level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for Resona {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: self.klass.clone(),
            color: self.color.clone(),
            level: self.level.clone(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: self.power.clone(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for Resona {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        // todo: 出現条件とタイミングがSkillにあるので詳細にパースする必要あり

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Resona,
            klass: OptionString::from_string(card_data[1].clone()),
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            level: OptionString::from_string(card_data[3].clone()),
            power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[8].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for Resona {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "種族\t:{}\n", self.klass)?;
        write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct ResonaCraft {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    klass: OptionString,
    color: String,
    level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for ResonaCraft {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: self.klass.clone(),
            color: self.color.clone(),
            level: self.level.clone(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: self.power.clone(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for ResonaCraft {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        // todo: 出現条件とタイミングがSkillにあるので詳細にパースする必要あり

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::ResonaCraft,
            klass: OptionString::from_string(card_data[1].clone()),
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            level: OptionString::from_string(card_data[3].clone()),
            power: OptionString::from_string(card_data[7].clone()),
            user: OptionString::from_string(card_data[8].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for ResonaCraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        write!(f, "種族\t:{}\n", self.klass)?;
        write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct ArtsCraft {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: String,
    // level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    // power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<Card> for ArtsCraft {
    fn into(self) -> Card {
        Card {
            no: self.no.clone(),
            name: self.name.clone(),
            pronounce: self.pronounce.clone(),
            artist: self.artist.clone(),
            card_type: self.card_type.clone(),
            klass: OptionString::empty(),
            color: self.color.clone(),
            level: OptionString::empty(),
            cost: self.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: self.user.clone(),
            time: self.time.clone(),
            story: self.story.clone(),
            format: self.format.clone(),
            rarity: self.rarity.clone(),
            skill: self.skill.clone(),
            features: self.features.clone(),
        }
    }
}

impl WixossCard for ArtsCraft {
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

        let (skill, features) = parse_card_skill(card_skill.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::ArtsCraft,
            color: card_data[2].clone(),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[1].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for ArtsCraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NO.\t:{}\n", self.no)?;
        write!(f, "Name\t:{}\n", self.name)?;
        write!(f, "読み\t:{}\n", self.pronounce)?;
        write!(f, "絵\t:{}\n", self.artist)?;
        write!(f, "Type\t:{}\n", self.card_type)?;
        write!(f, "色\t:{}\n", self.color)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        write!(f, "ストーリー\t:{}\n", self.story)?;
        write!(f, "フォーマット\t:{}\n", self.format)?;
        write!(f, "レアリティ\t:{}\n", self.rarity)?;
        write!(f, "テキスト({})\t:{}\n", self.skill.value.len(), self.skill)?;
        write!(f, "フィーチャー({})\t:{:?}\n", self.features.len(), self.features.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "")
    }
}


fn parse_card_skill(source: String) -> (Skills, HashSet<CardFeature>) {
    let re_br = Regex::new(r"<br\s?>").unwrap();
    let mut features: HashSet<CardFeature> = HashSet::new();
    // Skills::from_vec(
    let a = re_br
        .replace_all(&source, "\n")
        .split("\n")
        .map(|line| line.trim().to_string())
        .map(|line| {
            let (l, features_detected) = rule_explain_to_feature(line);
            features.extend(features_detected);
            l
        })
        .filter(|line| !line.is_empty())  // この行を追加して空の行を除去する
        .collect();
    // )

    (Skills::from_vec(a), features)
}

macro_rules! features {
    ($($feature:expr),* $(,)?) => {
        {
            let mut set = HashSet::new();
            $(
                set.insert($feature);
            )*
            set
        }
    };
}

fn rule_explain_to_feature(text: String) -> (String, Vec<CardFeature>) {
    let text = replace_img_with_alt(text);

    let mut features: Vec<CardFeature> = Vec::new();

    let remove_patterns: Vec<(&str, bool, &str, HashSet<CardFeature>)> = vec![
        (r"『", true, "", features![]),  // アクセのみ？
        (r"』", true, "", features![]),  // アクセのみ？
        (r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）", true, "", features![CardFeature::Damage]),
        (r"（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）", true, "", features![CardFeature::Lancer]),
        (r"（このクラフトは効果以外によっては場に出せない）", true, "", features![CardFeature::Craft]),
        (r"アクセ", false, "*ACCE*", features![CardFeature::Acce]),
        (r"（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）", true, "", features![CardFeature::Acce]),
        (r"（あなたのルリグの下からカードを合計４枚ルリグトラッシュに置く）", true, "*EXCEED*", features![CardFeature::Exceed]),
        (r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに合計１枚までしか入れられない）", true, "*DREAM TEAM*", features![]),
        (r"（あなたの場にいるルリグ３体がこの条件を満たす）", true, "*TEAM*", features![]),
        (r"（シグニは覚醒すると場にあるかぎり覚醒状態になる）", true, "*AWAKE*", features![CardFeature::Awake]),
        (r"（凍結されたシグニは次の自分のアップフェイズにアップしない）", true, "*FROZEN*", features![CardFeature::Freeze]),
        (r"（フェゾーネマジックは５種類ある）", true, "*FESONE MAGIC*", features![]),
        (r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）", true, "*CIP COST*", features![]),
        (r"ガードアイコン", true, "ガード", features![CardFeature::Guard]),
        (r"捨てさせる。", false, "*HAND DESTRUCTION*", features![CardFeature::DiscardOpponent]),
        (r"見ないで選び、捨てさせる。", false, "*RANDOM HAND DESTRUCTION*", features![CardFeature::RandomDiscard]),
        (r"ダウンする。", false, "*DOWN*", features![CardFeature::Down]),
        (r"エナチャージ", false, "*CHARGE*", features![CardFeature::Charge]),
        (r"残りを好きな順番でデッキの一番下に置く", false, "*BOTTOM CHECK*", features![CardFeature::BottomCheck]),
        (r"トラッシュに置", false, "*TRASH*", features![CardFeature::Trash]),
        (r"シグニバリア", false, "*BARRIER*", features![CardFeature::Barrier]),
        (r"ルリグバリア", false, "*BARRIER*", features![CardFeature::Barrier]),
        (r"がアタックしたとき", false, "*ON ATTACK*", features![CardFeature::OnAttack]),
        (r"アサシン", false, "*ASSASSIN*", features![CardFeature::Assassin]),
        (r"シャドウ", false, "*SHADOW*", features![CardFeature::Shadow]),
        (r"チャーム", false, "*CHARM*", features![CardFeature::Charm]),
        (r"ダブルクラッシュ", false, "*DOUBLE CRUSH*", features![CardFeature::DoubleCrush]),
        (r"トリプルクラッシュ", false, "*TRIPLE CRUSH*", features![CardFeature::TripleCrush]),
        (r"Sランサー", false, "*S LANCER*", features![CardFeature::SLancer]),
        (r"バニッシュ", false, "*BANISH*", features![CardFeature::Banish]),
        (r"凍結する", false, "*FREEZE*", features![CardFeature::Freeze]),
        (r"手札に戻す", false, "*BOUNCE*", features![CardFeature::Bounce]),
        (r"手札に加え", false, "*Salvage*", features![CardFeature::Salvage]),
        (r"ライフクロス[（\u{FF10}-\u{FF19}）]+枚をトラッシュに置", false, "*LIFE TRASH*", features![CardFeature::LifeTrash]),
        (r"エナゾーンからカード[（\u{FF10}-\u{FF19}）]+枚を.+トラッシュに置", false, "*ENER ATTACK*", features![CardFeature::EnerAttack]),
        (r"ルリグトラッシュに置", false, "*ENER ATTACK*", features![CardFeature::LrigTrash]),
        (r"アタックフェイズ開始時", false, "*ON ATTACK START*", features![CardFeature::OnAttackStart]),
        (r"ライフクロスに加える", false, "*ADD LIFE*", features![CardFeature::AddLife]),
        (r"ランサー", false, "*LANCER*", features![CardFeature::Lancer]),
        (r"ライフクロスを１枚クラッシュする", false, "*CRUSH*", features![CardFeature::LifeCrush]),
        (r"対戦相手にダメージを与える。", false, "*DAMAGE*", features![CardFeature::Damage]),
    ];

    let replaced_text = remove_patterns.iter().fold(text, |current_text, pat| {
        let re = Regex::new(pat.0).unwrap();

        if re.is_match(&current_text) {
            features.extend(pat.3.iter().cloned());
        }

        if pat.1 {
            re.replace_all(&current_text, pat.2).to_string()
        } else {
            current_text
        }
    });

    (replaced_text, features)
}


fn replace_img_with_alt(html: String) -> String {
    let re = Regex::new(r#"<img[^>]*alt="([^"]*)"[^>]*>"#).unwrap();
    let replaced = re.replace_all(&html, |caps: &regex::Captures| {
        let alt_text = &caps[1];
        alt_text.replace("2》", "》")
    });
    replaced.into_owned()
}

fn parse_story(html: String) -> OptionString {
    if html.contains(r##"class="cardData_story_img""##) {
        OptionString::from_string("ディソナ".into())
    } else {
        OptionString::empty()
    }
}

fn split_by_break(html: String) -> Vec<String> {
    html.replace("\n", "").split("<br>")
        .map(|s| s.to_string())
        .collect()
}

fn flatten_break(html: String) -> String {
    html.replace("\n", "").replace("<br>", "")
}

fn parse_format(html: String) -> Format {
    match html.as_str() {
        _ if html.contains("ディーヴァアイコン") => Format::DivaSelection,
        _ if html.contains("キーアイコン") => Format::KeySelection,
        _ => Format::AllStar,
    }
}
