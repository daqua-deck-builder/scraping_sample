use std::fmt::{Display, Formatter, write};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CardFeature {
    DoubleCrush,
    TripleCrush,
    DiscardOpponent,
    RandomDiscard,
    Draw,
    Assassin,
    Freeze,
    Drop,
    // DeckAttack,
    OnDrop,
    OnRefresh,
    Lancer,
    SLancer,
    Penetrate,
    NonAttackable,
    Down,
    Up,
    Charge,
    EnerAttack,
    Trash,
    PowerUp,
    PowerDown,
    Bounce,
    DeckBounce,
    Salvage,
    LifeBurst,
    Shadow,
    Invulnerable,
    OnSpell,
    OnArts,
    OnPiece,
    OnBanish,
    Guard,
    OnGuard,
    AttackNoEffect,
    OnAttack,
    OnAttackStart,
    OnTouch,
    Awake,
    Exceed,
    OnExceed,
    AddLife,
    OnBurst,
    LifeTrash,
    OnLifeCrush,
    Position,
    Vanilla,
    Untouchable,
    // アークゲイン
    TopCheck,
    BottomCheck,
}

impl Display for CardFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            CardFeature::DoubleCrush => "ダブルクラッシュ",
            CardFeature::TripleCrush => "トリプルクラッシュ",
            CardFeature::DiscardOpponent => "手札破壊",
            CardFeature::RandomDiscard => "ランダム手札破壊",
            CardFeature::Draw => "ドロー",
            CardFeature::Assassin => "アサシン",
            CardFeature::Freeze => "凍結",
            CardFeature::Drop => "デッキドロップ", // DeckAttack,
            CardFeature::OnDrop => "デッキドロップ時",
            CardFeature::OnRefresh => "リフレッシュ時",
            CardFeature::Lancer => "ランサー",
            CardFeature::SLancer => "Sランサー",
            CardFeature::Penetrate => "ガード不可",
            CardFeature::NonAttackable => "アタック不可",
            CardFeature::Down => "ダウン",
            CardFeature::Up => "アップ",
            CardFeature::Charge => "エナチャージ",
            CardFeature::EnerAttack => "エナ破壊",
            CardFeature::Trash => "トラッシュ",
            CardFeature::PowerUp => "パワーアップ",
            CardFeature::PowerDown => "パワーダウン",
            CardFeature::Bounce => "バウンス",
            CardFeature::DeckBounce => "デッキバウンス",
            CardFeature::Salvage => "回収",
            CardFeature::LifeBurst => "ライフバースト",
            CardFeature::Shadow => "シャドウ",
            CardFeature::Invulnerable => "バニッシュされない",
            CardFeature::OnSpell => "スペル使用時",
            CardFeature::OnArts => "アーツ使用時",
            CardFeature::OnPiece => "ピース使用時",
            CardFeature::OnBanish => "バニッシュした時",
            CardFeature::Guard => "ガード",
            CardFeature::OnGuard => "ガードした時",
            CardFeature::AttackNoEffect => "アタック無効",
            CardFeature::OnAttack => "アタックした時",
            CardFeature::OnAttackStart => "アタック開始時",
            CardFeature::OnTouch => "対象になった時",
            CardFeature::Awake => "覚醒",
            CardFeature::Exceed => "エクシード",
            CardFeature::OnExceed => "エクシードした時",
            CardFeature::AddLife => "ライフクロス追加",
            CardFeature::OnBurst => "ライフバースト発動時",
            CardFeature::LifeTrash => "ライフクロストラッシュ送り",
            CardFeature::OnLifeCrush => "クラッシュされた時",
            CardFeature::Position => "シグニゾーン移動",
            CardFeature::Vanilla => "能力を持たない",
            CardFeature::Untouchable => "効果を受けない", // アークゲイン
            CardFeature::TopCheck => "トップ確認",
            CardFeature::BottomCheck => "ボトム確認",
            _ => ""
        };
        write!(f, "{}", label)
    }
}