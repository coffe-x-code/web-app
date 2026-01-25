use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, get,
    http::header::{self, HeaderMap, HeaderValue},
    post, web,
};
use serde::Serialize;
use tera::{Context, Tera};
struct AppState {
    tera: Tera,
}
#[derive(Serialize)]
struct Product {
    id: i32,
    name: String,
    description: String,
    image_url: String,
}
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                tera: Tera::new("./templates/**").unwrap(),
            }))
            .service(web::scope("/app").service(index))
            .service(css)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let context = Context::new();
    let template_name = "home.html";
    return HttpResponse::Ok().body(data.tera.render(template_name, &context).unwrap());
}
#[get("/css")]
pub async fn css() -> impl Responder {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("text/css").unwrap(),
    );
    let css = std::fs::read_to_string("output/style.css").unwrap();
    return HttpResponse::Ok().body(css);
}
#[post("/search")]
pub async fn search(data: web::Data<AppState>) -> impl Responder {
    let search_result_template_name: String = String::from("search_result.html");
    let product1 = Product {
        id: 32,
        name: String::from("protein"),
        description: String::from("supplement"),
        image_url: String::from(
            "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.verywellhealth.com%2Fthmb%2FP6T-yEHRfWJdtHoZXYdaYGjKSzc%3D%2F2121x1414%2Ffilters%3Ano_upscale()%3Amax_bytes(150000)%3Astrip_icc()%2FGettyImages-1145581060-c6f3afa5f308461cab0a77d79a51c68a.jpg&f=1&nofb=1&ipt=067d1f601849fc9308e22677dab44d26c694c8c2513a5d468e6ee7d9de251a8e",
        ),
    };
    let product2 = Product {
        id: 31,
        name: String::from("magnesium"),
        description: String::from("supplement for better sleep"),
        image_url: String::from(
            "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.verywellhealth.com%2Fthmb%2FP6T-yEHRfWJdtHoZXYdaYGjKSzc%3D%2F2121x1414%2Ffilters%3Ano_upscale()%3Amax_bytes(150000)%3Astrip_icc()%2FGettyImages-1145581060-c6f3afa5f308461cab0a77d79a51c68a.jpg&f=1&nofb=1&ipt=067d1f601849fc9308e22677dab44d26c694c8c2513a5d468e6ee7d9de251a8e",
        ),
    };
    let mut ctx = Context::new();
    let mut vec: Vec<Product> = Vec::new();
    vec.push(product1);
    vec.push(product2);
    ctx.insert("products", &vec);
    return HttpResponse::Ok().body(
        data.tera
            .render(&search_result_template_name, &ctx)
            .unwrap(),
    );
}
