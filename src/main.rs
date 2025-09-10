use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod handlers;
use handlers::{hello, Mensagem};

#[derive(OpenApi)]
#[openapi(paths(handlers::hello), components(schemas(Mensagem)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Servidor rodando em http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
