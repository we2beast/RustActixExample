use actix_web::{web, HttpResponse};

fn index() -> HttpResponse {
    http_response("Text example for starting page")
}

fn http_response(body: &'static str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}