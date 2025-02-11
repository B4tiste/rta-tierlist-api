mod monsters;
mod router;
mod tierlists;

use mongodb::{Client, Database};
use router::create_routes;
use shuttle_runtime::{CustomError, SecretStore};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    let mongodb_uri = secret_store
        .get("MONGODB_URI")
        .ok_or_else(|| CustomError::msg("Can't find MONGODB_URI in Secrets.toml"))?;

    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .map_err(|e| CustomError::msg(e.to_string()))?;

    let db = client.database("rta-tierlists-db");

    let app_state = Arc::new(AppState { db });

    let rocket = rocket::build().manage(app_state);

    let rocket = create_routes(rocket);

    Ok(rocket.into())
}
