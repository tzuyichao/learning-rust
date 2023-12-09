use actix_web::{web, App, HttpResponse, HttpServer}

fn main() {
    let server =  HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    println!("Server is running on http://localhost:3000");
    server.bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <h1>It works</h1>
            "#,
        );
}
