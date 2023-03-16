use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter5/handlers/mod.rs"]
mod handlers;

#[path = "../iter5/models/mod.rs"]
mod models;

#[path = "../iter5/routes.rs"]
mod routes;

#[path = "../iter5/state.rs"]
mod state;

#[path = "../iter5/dbaccess/mod.rs"]
mod db_access;

#[path = "../iter5/errors.rs"]
mod errors;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL was not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    let host_port = env::var("HOST_PORT").expect("HOST:PORT not set in .env file");

    HttpServer::new(app).bind(host_port)?.run().await
}

// use core::fmt;
// use std::fs::File;
// use std::io::Write;

// #[derive(Debug)]
// pub enum MyError {
//     ParseError,
//     IOError,
// }

// impl std::error::Error for MyError {}

// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             MyError::ParseError => write!(f, "Parse Error"),
//             MyError::IOError => write!(f, "IO Error"),
//         }
//     }
// }

// fn main() {
//     let result = square("INVALID");
//     match result {
//         Ok(res) => println!("Result is {}", res),
//         Err(e) => println!("Error in parsing: {:?}", e),
//     };
// }

// fn square(val: &str) -> Result<i32, MyError> {
//     let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
//     let mut f = File::open("fictionalfile.txt").map_err(|_| MyError::IOError)?;
//     let string_to_write = format!("Square of {} is {}", num, i32::pow(num, 2));
//     f.write_all(string_to_write.as_bytes())
//         .map_err(|_| MyError::IOError)?;
//     Ok(i32::pow(num, 2))
// }
