#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() {
    rocket::ignite()
    .mount("/static", StaticFiles::from("../pages"))
    .launch();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}