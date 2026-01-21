use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::header::{self, HeaderMap, HeaderValue}, web::{self}};
use tera::{Context, Tera};
  

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
         App::new().service(
            web::scope("/app")
                .service(index)  
        ).service(css)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[get("")]
pub async fn index() -> impl Responder {
   let context = Context::new();
   let template_name ="home.html";

   let tera = Tera::new("./templates/**").unwrap();
   return  HttpResponse::Ok().body(tera.render(template_name, &context).unwrap());
}
#[get("/css")]
pub async fn css() -> impl Responder {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_str("text/css").unwrap());
    let css = std::fs::read_to_string("templates/assets/tailwind.css").unwrap();
    return  HttpResponse::Ok().body(css);
 
}