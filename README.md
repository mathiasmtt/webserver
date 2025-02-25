# Servidor Web en Rust

Un servidor web moderno y eficiente construido con Rust y Actix-web, que demuestra las capacidades de Rust para el desarrollo web.

## Características
- 🚀 Alto rendimiento gracias a la eficiencia de Rust
- 🛡️ Seguridad garantizada por el sistema de tipos de Rust
- 🌐 API moderna basada en Actix-web
- 📱 Interfaz de usuario responsive y moderna
- ⏰ Muestra en tiempo real la hora del servidor
- 🎨 Diseño moderno con TailwindCSS y FontAwesome

## Tecnologías Utilizadas
- Rust
- Actix-web (Framework web)
- TailwindCSS (Framework CSS)
- FontAwesome (Iconos)
- Tera (Sistema de templates)

## Requisitos Previos
- Rust y Cargo instalados en tu sistema

## Instalación

1. Clona el repositorio:
```bash
git clone https://github.com/mathiasmtt/webserver.git
cd webserver
```

2. Ejecuta el servidor:
```bash
cargo run --release
```

3. Abre tu navegador y visita:
```
http://localhost:8080
```

## Estructura del Proyecto
```
webserver/
├── src/
│   └── main.rs          # Código fuente principal
├── templates/
│   └── index.html       # Plantilla principal con diseño moderno
├── static/              # Directorio para archivos estáticos
├── Cargo.toml           # Configuración y dependencias
└── README.md            # Documentación
```

## Características Detalladas

### Frontend
- Diseño moderno y responsive usando TailwindCSS
- Iconografía con FontAwesome
- Navegación suave entre secciones
- Visualización en tiempo real de la hora del servidor

### Backend
- Servidor web de alto rendimiento con Actix-web
- Sistema de templates con Tera
- Manejo eficiente de rutas y recursos estáticos
- Arquitectura modular y extensible

## Contribución
Las contribuciones son bienvenidas. Por favor:
1. Haz fork del repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Realiza commit de tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## Licencia
Este proyecto está bajo la Licencia MIT.

## Autor
Mathias
