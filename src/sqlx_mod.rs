use rocket_db_pools::sqlx::{ self, Row };
use rocket_db_pools::{ Connection, Database };

use rocket::{ Build, Rocket };

use rocket::fairing::{ self, AdHoc };
use rocket::serde::{ Deserialize, Serialize };

#[derive(Database)]
#[database("data")]
pub struct Data(sqlx::SqlitePool);


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Data::fetch(&rocket) {
        Some(db) =>
            match sqlx::migrate!("./migrations").run(&**db).await {
                Ok(_) => Ok(rocket),
                Err(e) => {
                    error!("Failed to initialize SQLx database: {}", e);
                    Err(rocket)
                }
            }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Data::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/api/v1", routes![])
    })
}
