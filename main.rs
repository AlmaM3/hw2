/*use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}*/

use actix_web::{post, web, HttpResponse, HttpServer, App, HttpRequest, Responder};
use serde::{Serialize, Deserialize};
//use std::process::Command;

#[derive(Deserialize)]
pub struct Request {
    rfc: String,
}

#[derive(Serialize)]
pub struct Response {
    result: bool,
}

#[post("/")]
    pub async fn rfc(req_body:String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
        //format!("validating password for {}", request.rfc);
    }


async fn protegido(req: HttpRequest) -> impl Responder {
    let process = req.rfc().post::new("sh");
        /*.arg(&request.Command)
        .status()
        .expect("Failed to access rfc");*/

    println!("status: {}", &process.to_string());

    if process.success() {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(protegido))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
