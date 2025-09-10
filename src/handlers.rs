use actix_web::{get, web, Responder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Mensagem {
    pub mensagem: String,
}

#[get("/hello/{nome}")]
pub async fn hello(nome: web::Path<String>) -> impl Responder {
    let resposta = Mensagem {
        mensagem: format!("Olá, {}!", nome),
    };
    web::Json(resposta)
}
