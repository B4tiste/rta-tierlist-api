use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::tierlists::tierlists_models::TierList;

pub struct TierlistsService {
    pub collection: Collection<TierList>,
}

impl TierlistsService {
    pub async fn get_tierlist(&self, id: ObjectId) -> Option<TierList> {
        println!("zazou");
        let count = self.collection.count_documents(doc! {}).await.unwrap();
        println!("Count: {:?}", count);
        self.collection.find_one(doc! { "_id": id }).await.unwrap()
    }
}
