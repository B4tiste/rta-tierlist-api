use rocket::{Build, Rocket};

use crate::tierlists::tierlists_controller;

pub fn create_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/tierlists", tierlists_controller::create_routes())
}
