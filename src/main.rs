// package dependency
use rocket::launch;

// module dependency
use rocket_prac::fn_lib::launch_rocket;

#[launch]
pub fn rocket() -> _ {
    launch_rocket()
}
