// package dependency
use rocket::{routes, Build, Rocket};

// mod dependency
use crate::route_lib::*;

pub fn launch_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/test_conn", routes![test_conn])
        .mount("/test_read", routes![test_read])
}
