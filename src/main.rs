use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use serde::{Deserialize, Serialize};
// use rust_bert::pipelines::text_generation::TextGenerationModel;
// use rust_bert::pipelines::common::ModelType;


#[get("/")]
async fn index() -> impl Responder {
    println!("hello");  // This is printed in the console for test
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    output: String,
}

//  Natural Language Generation function
#[derive(Debug, Serialize, Deserialize)]
struct GenerateRequest {
    prompt: String,
}


#[post("/generator")]
async fn generator(info: web::Json<GenerateRequest>) ->  impl Responder {
    println!("gogogo");
    let prompt = info.prompt.to_owned();
    println!("prompt: {}", prompt);
    // let model = TextGenerationModel::new(Default::default());
    // let output = model.expect("REASON").generate(&[prompt], None);
    // let output = output[0].to_owned();
    let output = format!("{}{}", "prefix", prompt);
    let reponse = Response { output };

    HttpResponse::Ok().json(reponse)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(generator)
            .service(Files::new("/static", "./templates"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
