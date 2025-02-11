use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TierList {
    pub _id: ObjectId,
    pub elo: Elo,
    pub tierlist: Vec<RangTierlist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangTierlist {
    pub rank: Rank,
    pub list_monsters: Vec<MonsterInfoTierlist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoTierlist {
    pub score: f64,
    pub id_monster: i64,
    pub winrate: f64,
    pub pickrate: f64,
    pub banrate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Monster {
    pub _id: i64,
    name: String,
    img_url: String,
    element: Element,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Elo {
    Conq,
    G1,
    G3,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Rank {
    sssMonster,
    ssMonster,
    smonster,
    amonster,
    bmonster,
    cmonster,
    dmonster,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Element {
    Fire,
    Water,
    Wind,
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedMonsterInfo {
    pub score: f64,
    pub monster: Monster, // full monster details
    pub winrate: f64,
    pub pickrate: f64,
    pub banrate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedRangTierlist {
    pub rank: Rank,
    pub list_monsters: Vec<DetailedMonsterInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TierListDetailed {
    pub _id: ObjectId,
    pub elo: Elo,
    pub tierlist: Vec<DetailedRangTierlist>,
}
