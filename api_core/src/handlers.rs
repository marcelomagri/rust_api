use actix_web::{get, web, Responder};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Mensagem {
    pub mensagem: String,
}

#[utoipa::path(
    get,
    path = "/hello/{nome}",
    params(
        ("nome" = String, Path, description = "Nome da pessoa")
    ),
    responses(
        (status = 200, description = "Mensagem de boas-vindas", body = Mensagem)
    )
)]
#[get("/hello/{nome}")]
pub async fn hello(nome: web::Path<String>) -> impl Responder {
    let resposta = Mensagem {
        mensagem: format!("Olá, {}!", nome),
    };
    web::Json(resposta)
}

#[derive(OpenApi)]
#[openapi(
    paths(hello),
    components(schemas(Mensagem))
)]
pub struct ApiDoc;
