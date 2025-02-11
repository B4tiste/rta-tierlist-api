use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TierList {
    _id: ObjectId,
    elo: Elo,
    tierlist: Vec<RangTierlist>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RangTierlist {
    rank: Rank,
    list_monsters: Vec<MonsterInfoTierlist>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MonsterInfoTierlist {
    score: f64,
    id_monster: i64,
    winrate: f64,
    pickrate: f64,
    banrate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Monster {
    _id: i64,
    name: String,
    img_url: String,
    element: Element,
}

#[derive(Debug, Serialize, Deserialize)]
enum Elo {
    Conq,
    G1,
    G3,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
enum Rank {
    sssMonster,
    ssMonster,
    smonster,
    amonster,
    bmonster,
    cmonster,
    dmonster,
}

#[derive(Debug, Serialize, Deserialize)]
enum Element {
    Fire,
    Water,
    Wind,
    Light,
    Dark,
}
