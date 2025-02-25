use actix_web::{get, App, HttpResponse, HttpServer, Result};
use actix_files as fs;
use tera::{Tera, Context};
use std::time::SystemTime;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Error al parsear templates: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error en el template"));
        }
    };
    
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut context = Context::new();
    context.insert("current_time", &current_time.to_string());
    
    let rendered = match tera.render("index.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error al renderizar template: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al renderizar"));
        }
    };
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor iniciado en http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
