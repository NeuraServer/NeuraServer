#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use models::{User, NewUser};

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    use schema::users;

    let connection = establish_connection();
    diesel::insert_into(users::table)
        .values(&user.into_inner())
        .execute(&connection)
        .expect("Error saving new user");

    HttpResponse::Ok().body("User created")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    use schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    HttpResponse::Ok().json(results)
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(get_users)
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}
