use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn cadastro(req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    let params: Vec<(&str, &str)> = url::form_urlencoded::parse(query.as_bytes()).collect();
    let mut nome = "";
    let mut sobrenome = "";
    for (key, value) in params {
        if key == "nome" {
            nome = value;
        } else if key == "sobrenome" {
            sobrenome = value;
        }
    }
    format!("Nome: {}<br>Sobrenome: {}", nome, sobrenome)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/code/cadastro.rs", web::get().to(cadastro))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
