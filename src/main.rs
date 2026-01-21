use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web::{self, Data}};
use tera::{Context, Tera};
  

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
         App::new().service(
            web::scope("/app")
                .service(index)
        )
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

       