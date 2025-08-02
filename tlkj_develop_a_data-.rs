// Import necessary libraries
extern crate actix_web;
extern crate diesel;
extern crate serde;

use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};

// Define database connection and schema
mod schema {
    table! {
        users (id) {
            id -> Integer,
            name -> VarChar,
            email -> VarChar,
        }
    }
}

// Define data model
#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Define API routes
async fn get_users() -> impl Responder {
    let connection = PgConnection::establish("database_url").unwrap();
    let results = web::block(move || {
        schema::users::table.load::<User>(&connection)
    }).await.unwrap();
    web::Json(results)
}

async fn get_user(id: web::Path<i32>) -> impl Responder {
    let connection = PgConnection::establish("database_url").unwrap();
    let result = web::block(move || {
        schema::users::table.find(id.into_inner()).first::<User>(&connection)
    }).await.unwrap();
    web::Json(result)
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    let connection = PgConnection::establish("database_url").unwrap();
    web::block(move || {
        diesel::insert_into(schema::users::table)
            .values(user.into Inner())
            .execute(&connection)
    }).await.unwrap();
    web::Json(user)
}

// Define dashboard route
async fn dashboard() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the data-driven web app dashboard!")
}

// Define main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/users").route(web::get().to(get_users)))
            .service(web::resource("/users/{id}").route(web::get().to(get_user)))
            .service(web::resource("/create-user").route(web::post().to(create_user)))
            .service(web::resource("/dashboard").route(web::get().to(dashboard)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}