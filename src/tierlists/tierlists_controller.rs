use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use rocket::serde::json::Json;
use rocket::{get, http::Status, routes, Route, State};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::tierlists::tierlists_models::{
    DetailedMonsterInfo, DetailedRangTierlist, Monster, TierList, TierListDetailed,
};
use crate::tierlists::tierlists_service::{MonsterService, TierlistsService};
use crate::AppState;

#[get("/<id>")]
pub async fn get_tierlists(
    id: &str,
    state: &State<Arc<AppState>>,
) -> Result<Json<TierListDetailed>, Status> {
    // Conversion de la chaîne en ObjectId
    let object_id = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;

    // Récupération de la tierlist
    let tierlists_service = TierlistsService {
        collection: state.db.collection::<TierList>("tierlists"),
    };

    let tierlist = tierlists_service
        .get_tierlist(object_id)
        .await
        .ok_or(Status::NotFound)?;

    // 1. Extraction des id_monster uniques de la tierlist
    let mut ids_set = HashSet::new();
    for rang in &tierlist.tierlist {
        for monster_info in &rang.list_monsters {
            ids_set.insert(monster_info.id_monster);
        }
    }
    let ids: Vec<i64> = ids_set.into_iter().collect();

    // 2. Requête unique pour récupérer tous les monstres avec un $in
    let monster_service = MonsterService {
        collection: state.db.collection::<Monster>("monsters"),
    };

    let filter = doc! { "_id": { "$in": ids } };
    let mut cursor = monster_service
        .collection
        .find(filter)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut monsters = Vec::new();
    while let Some(monster) = cursor
        .try_next()
        .await
        .map_err(|_| Status::InternalServerError)?
    {
        monsters.push(monster);
    }

    // 3. Construction d'une HashMap pour associer id -> Monster
    let monsters_map: HashMap<i64, Monster> = monsters.into_iter().map(|m| (m._id, m)).collect();

    // 4. Construction de la réponse détaillée
    let mut detailed_tierlist = TierListDetailed {
        _id: tierlist._id,
        elo: tierlist.elo,
        tierlist: Vec::new(),
    };

    for rang in tierlist.tierlist {
        let mut detailed_monsters = Vec::new();

        for monster_info in rang.list_monsters {
            if let Some(monster) = monsters_map.get(&monster_info.id_monster) {
                detailed_monsters.push(DetailedMonsterInfo {
                    score: monster_info.score,
                    monster: (*monster).clone(),
                    winrate: monster_info.winrate,
                    pickrate: monster_info.pickrate,
                    banrate: monster_info.banrate,
                });
            }
        }

        detailed_tierlist.tierlist.push(DetailedRangTierlist {
            rank: rang.rank,
            list_monsters: detailed_monsters,
        });
    }

    Ok(Json(detailed_tierlist))
}

pub fn create_routes() -> Vec<Route> {
    routes![get_tierlists]
}
