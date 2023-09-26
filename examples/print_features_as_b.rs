#[macro_use]
use scraping_sample::wixoss::constants::constants::CardFeature;
use std::collections::HashSet;
use std::fmt::{Display};

fn main() {
    let feature_set: HashSet<CardFeature> = vec![
        CardFeature::DoubleCrush,
        CardFeature::Damage,
    ].into_iter().collect();

    println!("{:b}", hashset_to_flags(feature_set));
}

fn hashset_to_flags(feature_set: HashSet<CardFeature>) -> u64 {
    let mut b: u64 = 0;
    feature_set.iter().for_each(|f| {
        b = b | f.to_bit()
    });
    b
}