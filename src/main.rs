#[macro_use] extern crate rocket;
mod lib;
use rocket::form::{Form, Contextual, FromForm, Context};
use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;
use rocket::fs::{FileServer, relative};
use rust_bert::pipelines::translation::Language;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct Submit<'v> {
    #[field(validate = len(1..=250))]
    r#submission: &'v str,
}

#[get("/")]
pub async fn health() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Health check passed.";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[get("/translate")]
async fn index() -> Template {
    Template::render("index", &Context::default())
}

// NOTE: We use `Contextual` here because we want to collect all submitted form
// fields to re-render forms with submitted values on error. If you have no such
// need, do not use `Contextual`. Use the equivalent of `Form<Submit<'_>>`.
#[post("/translate", data = "<form>")]
async fn submit<'r>(form: Form<Contextual<'r, Submit<'r>>>) -> (Status, Template) {
    let translation_model = lib::init_translation_model();
    let sanity_check = translation_model.translate(&["Hello world"], None, Language::Spanish).unwrap();
    println!("sanity check: {:#?}", sanity_check);

    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission.submission);

            let translations = translation_model.translate(&[submission.submission], None, Language::Spanish).unwrap();
            let full_translations = translations.join("\n");
            
            let response_json = &GenericResponse {
                status: "success".to_string(),
                message: full_translations.to_string(),
            };
            Template::render("success", response_json)

            
            // let response = dialogue(String::from(submission.submission));
            // Template::render("success", &response)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit, health])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}