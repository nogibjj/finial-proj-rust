use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use rust_bert::pipelines::text_generation::TextGenerationModel;
use rust_bert::pipelines::common::ModelType;


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

//  Natural Language Generation function
#[derive(serde::Deserialize)]
struct FormData {
    prompt: String,
}

#[get("/text_generator")]
async fn show_generator() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/generate.html"))
}

#[post("/generator")]
async fn generator(form: web::Form<FormData>) -> HttpResponse {
    let prompt = form.prompt.to_owned();
    let model = TextGenerationModel::new(Default::default());
    let output = model.expect("REASON").generate(&[prompt], None);
    let output = output[0].to_owned();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("{}", output))
}









#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(show_generator)
            .service(generator)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
