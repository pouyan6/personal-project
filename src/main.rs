mod model;
mod handlers;

use actix_web::{middleware, web, App, HttpServer};
use mongodb::Client;
use std::env;
use actix_web::dev::Service;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::get_soldiers,
        handlers::add_soldier
    ),
    components(
        schemas(model::Soldier)
    ),
    tags(
        (name = "soldier", description = "Soldier management API")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://root:example@localhost:27017".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "personal".to_string());
    
    let client = Client::with_uri_str(uri).await.expect("failed to connect to mongodb");

    println!("Starting server on 127.0.0.1:8080");
    println!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/");

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap_fn(|req,srv| { // custom middleware example
                req.headers().get("secret-token");
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    Ok(res) }
            })
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(db_name.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(handlers::get_soldiers)
            .service(handlers::add_soldier)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
