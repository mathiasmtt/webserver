use actix_web::{get, App, HttpResponse, HttpServer, Result};
use actix_files as fs;
use tera::{Tera, Context};
use std::time::SystemTime;
use once_cell::sync::Lazy;

// Crear una instancia global de Tera
static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    println!("Inicializando Tera...");
    
    // Obtener la ruta absoluta al directorio de templates
    let current_dir = std::env::current_dir().unwrap();
    let template_dir = current_dir.join("templates");
    let template_pattern = format!("{}/**/*.html", template_dir.to_str().unwrap());
    
    println!("Buscando templates en: {}", template_pattern);
    
    // Verificar si los archivos existen
    println!("Verificando existencia de templates:");
    for entry in std::fs::read_dir(&template_dir).unwrap() {
        let entry = entry.unwrap();
        println!("  - {}", entry.path().display());
    }
    // Cargar los templates
    let tera = match Tera::new(&template_pattern) {
        Ok(t) => {
            println!("Templates cargados correctamente:");
            for template in t.get_template_names() {
                println!("  - {}", template);
            }
            t
        },
        Err(e) => {
            println!("Error al parsear templates: {}", e);
            ::std::process::exit(1);
        }
    };
    tera
});

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let mut context = Context::new();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    context.insert("current_time", &current_time.to_string());
    
    let rendered = match TEMPLATES.render("index.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error al renderizar template: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al renderizar"));
        }
    };
    
    println!("Template index.html renderizado correctamente");
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/about")]
async fn about() -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "Acerca de Nosotros");
    context.insert("description", "Somos un equipo apasionado por Rust y el desarrollo web moderno.");
    
    let rendered = match TEMPLATES.render("about.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error al renderizar template about.html: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al renderizar"));
        }
    };
    
    println!("Template about.html renderizado correctamente");
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/contact")]
async fn contact() -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "Contacto");
    context.insert("email", "info@rustserver.com");
    
    let rendered = match TEMPLATES.render("contact.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error al renderizar template contact.html: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Error al renderizar"));
        }
    };
    
    println!("Template contact.html renderizado correctamente");
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor iniciado en http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(about)
            .service(contact)
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
