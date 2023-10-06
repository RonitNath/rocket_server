use crate::sqlx_mod::Data;

use rocket_db_pools::Connection;

use rocket_dyn_templates::{Template, context};

pub mod routes {
    pub use super::{ index, about };
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        title: "Home",
    })
}

#[get("/about")]
pub async fn about(db: Connection<Data>) -> Template {
    // let people = sqlx_mod::get_all(db).await;

    Template::render("about", context! {
        title: "About",
        // people,
    })
}
