use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::tierlists::tierlists_models::{Monster, TierList};

pub struct TierlistsService {
    pub collection: Collection<TierList>,
}

pub struct MonsterService {
    pub collection: Collection<Monster>,
}

impl TierlistsService {
    pub async fn get_tierlist(&self, id: ObjectId) -> Option<TierList> {
        self.collection.find_one(doc! { "_id": id }).await.unwrap()
    }
}
