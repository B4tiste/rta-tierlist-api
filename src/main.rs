mod router;
mod tierlists;

use mongodb::{Client, Database};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
use router::create_routes;
use shuttle_runtime::{CustomError, SecretStore};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Database,
}

// Fonction pour configurer CORS
fn create_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Autoriser tous les domaines
        allowed_methods: vec!["GET".parse().unwrap(), "POST".parse().unwrap()]
            .into_iter()
            .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Erreur lors de la configuration de CORS")
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

    let db = client.database("rta-tierlist-db");

    let app_state = Arc::new(AppState { db });

    let rocket = rocket::build().manage(app_state).attach(create_cors()); // 🔥 Ajout de CORS ici

    let rocket = create_routes(rocket);

    Ok(rocket.into())
}
