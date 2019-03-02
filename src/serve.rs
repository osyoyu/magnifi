use actix_web::{server, App, HttpRequest, HttpResponse, Result};
use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::Logger;

use crate::searcher;

pub fn serve(bind_addr: String) {
    let sys = actix::System::new("magnifi");

    let _addr = server::new(||
        App::new()
            .middleware(Logger::default())
            .resource("/search", |r|
                r.method(Method::GET).f(search))
    )
    .bind(&bind_addr).expect(&format!("Could not bind to {}", bind_addr))
    .shutdown_timeout(0)
    .start();

    println!("Starting server on {}", bind_addr);
    let _ = sys.run();
}

fn search(req: &HttpRequest) -> Result<HttpResponse> {
    let params = req.query();
    let query = params.get("q");

    match query {
        Some(q) => {
            let result = searcher::search(q.to_string());
            Ok(HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .body(result)
            )
        },
        None => {
            Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json")
                .body("{}")
            )
        },
    }
}
