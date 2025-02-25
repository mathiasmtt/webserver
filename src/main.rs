use actix_web::{get, App, HttpResponse, HttpServer, Result};
use actix_files as fs;
use tera::{Tera, Context};
use std::time::SystemTime;
use std::path::Path;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    // Inicializar Tera con una instancia global o por solicitud
    let mut tera = Tera::default();
    
    // Obtener la ruta absoluta al directorio de templates
    let current_dir = std::env::current_dir().unwrap();
    let template_dir = current_dir.join("templates");
    
    // Registrar el template específico que queremos usar
    let template_path = template_dir.join("index.html");
    let template_path_str = template_path.to_str().unwrap();
    
    println!("Ruta del template: {}", template_path_str);
    
    // Verificar si el archivo existe
    if !Path::new(template_path_str).exists() {
        println!("¡Error! El archivo de template no existe en: {}", template_path_str);
        return Ok(HttpResponse::InternalServerError().body("Template no encontrado"));
    }
    
    // Cargar el template directamente desde el archivo
    match tera.add_template_file(template_path_str, Some("index.html")) {
        Ok(_) => println!("Template cargado correctamente"),
        Err(e) => {
            println!("Error al cargar el template: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al cargar el template"));
        }
    }
    
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
    
    println!("Template renderizado correctamente");
    
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
