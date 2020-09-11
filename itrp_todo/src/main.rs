use actix_web::{get, HttpResponse, HttpServer, App, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
