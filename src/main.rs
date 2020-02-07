extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Salary {
    last_name: String,
    first_name: String,
    middle_name: String,
    dept: String,
    group: String,
    compensation: f64,
}

async fn get_salaries() -> Result<()> {
    let conn = Connection::open("salaries.db")?;

    let mut stmt = conn.prepare("select * from Year2019",)?;

    let salaries = stmt.query_map(NO_PARAMS, |row| {
        Ok(
            Salary {
                last_name: row.get(0)?,
                first_name: row.get(1)?,
                middle_name: row.get(2)?,
                dept: row.get(3)?,
                group: row.get(4)?,
                compensation: row.get(5)?,
            }
        )
    })?;

    let mut return_vector: Vec<Salary> = Vec::new();

    for salary in salaries{
        return_vector.push(salary?);
    }

    for thing in return_vector{
        println!
    }

    return Ok(());
}

#[get("/data/{year}")]
async fn data_route(url_params: web::Path<u32>) -> Result<HttpResponse>{
    let _x = get_salaries().await.unwrap();



    return format!("THIS IS THE YEAR OF {}", url_params);
}

#[get("/")]
async fn index() -> impl Responder {
    format!("It's snowing outside!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(data_route)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}