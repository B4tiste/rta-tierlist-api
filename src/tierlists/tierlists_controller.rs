use bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::{get, http::Status, routes, Route, State};
use std::sync::Arc;

use crate::tierlists::tierlists_models::TierList;
use crate::tierlists::tierlists_service::TierlistsService;
use crate::AppState;

#[get("/<id>")]
pub async fn get_tierlists(
    id: &str,
    state: &State<Arc<AppState>>,
) -> Result<Json<TierList>, Status> {
    // Conversion de l'ID
    let object_id = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;

    // Création du service
    let tierlists_service = TierlistsService {
        collection: state.db.collection::<TierList>("tierlists"),
    };

    // Récupération de la tierlist sans enveloppe de réponse standard
    let tierlist = tierlists_service
        .get_tierlist(object_id)
        .await
        .ok_or_else(|| Status::NotFound)?;

    Ok(Json(tierlist))
}

pub fn create_routes() -> Vec<Route> {
    routes![get_tierlists]
}
