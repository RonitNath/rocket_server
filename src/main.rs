
#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::fs::{FileServer, relative};

use rocket_dyn_templates::{Template, context};


mod pages;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

mod sqlx_mod;

#[launch]
fn rocket() -> _ {
    use pages::routes::*;
    rocket::build().mount("/", FileServer::from(relative!("basic")))
        .attach(Template::fairing())
        .attach(sqlx_mod::stage())
        .mount("/", routes![index, about])
        .register("/", catchers![not_found])
}
