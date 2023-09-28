use serde;
use serde_json;

macro_rules! my_macro {
    ($group:ident, [$(($name:ident, $desc:expr, $bit:expr)),*$(,)?]) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub enum $group {
            $(
                $name,
            )*
        }

        impl std::fmt::Display for $group {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $group::$name => write!(f, "{}", $desc),
                    )*
                }
            }
        }

        impl $group {
            pub fn to_bit(&self) -> u64 {
                match self {
                    $(
                        $group::$name => 1 << $bit,
                    )*
                }
            }
            pub fn to_bit_index() -> serde_json::Value {
                let mut map = serde_json::Map::new();
                $(
                    let mut inner_map = serde_json::Map::new();
                    inner_map.insert("label".to_string(), serde_json::Value::String($desc.to_string()));
                    inner_map.insert("b".to_string(), serde_json::Value::Number(serde_json::Number::from($bit)));
                    map.insert(stringify!($name).to_string(), serde_json::Value::Object(inner_map));
                )*
                serde_json::Value::Object(map)
            }
        }
    }
}

fn main() -> Result<(), ()> {
my_macro!(CardFeature1, [
    (DoubleCrush, "ダブルクラッシュ", 0),
    (TripleCrush, "トリプルクラッシュ", 1),
    (DiscardOpponent, "手札破壊", 2),
]);

my_macro!(CardFeature2, [
    (RandomDiscard, "ランダム手札破壊", 0),
    (Draw, "ドロー", 1),
]);

    let index1 = CardFeature1::to_bit_index();
    println!("{}", serde_json::to_string_pretty(&index1).unwrap());
    let index2 = CardFeature2::to_bit_index();
    println!("{}", serde_json::to_string_pretty(&index2).unwrap());
    Ok(())
}