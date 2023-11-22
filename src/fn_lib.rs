// package dependency
use rocket::{routes, Rocket, Build};

// mod dependency
use crate::route_lib::index;

pub fn launch_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}