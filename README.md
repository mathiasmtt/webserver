# Servidor Web en Rust

Un servidor web moderno y eficiente construido con Rust y Actix-web, que demuestra las capacidades de Rust para el desarrollo web. Este proyecto combina la potencia y seguridad de Rust en el backend con una interfaz de usuario moderna y adaptable.

## 🚀 Características

### Backend (Rust)
- ⚡ Alto rendimiento gracias al sistema de actores de Actix-web
- 🛡️ Seguridad garantizada por el sistema de tipos de Rust
- 🔄 Manejo eficiente de solicitudes concurrentes
- ⏰ Servidor de tiempo real para mostrar la hora del servidor
- 🎯 Zero-cost abstractions de Rust para máximo rendimiento

### Frontend
- 🎨 Diseño moderno y responsive con TailwindCSS
- 🌓 Toggle de tema claro/oscuro con persistencia
- 📱 Interfaz adaptativa para todos los dispositivos
- ⚡ Transiciones y animaciones suaves
- 🔍 Navegación intuitiva con scroll suave

## 🛠️ Tecnologías Utilizadas

### Backend
- **Rust** - Lenguaje de programación seguro y eficiente
- **Actix-web** - Framework web de alto rendimiento
- **Tera** - Motor de plantillas para Rust
- **Tokio** - Runtime asíncrono para Rust

### Frontend
- **TailwindCSS** - Framework CSS utilitario
- **FontAwesome** - Iconografía moderna
- **LocalStorage API** - Persistencia de preferencias de usuario
- **JavaScript** - Interactividad y gestión del tema

## 📋 Requisitos Previos
- Rust y Cargo instalados en tu sistema
- Navegador web moderno con JavaScript habilitado

## 🚀 Instalación

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

## 📁 Estructura del Proyecto
```
webserver/
├── src/
│   └── main.rs          # Código del servidor Rust
├── templates/
│   └── index.html       # Plantilla principal con UI moderna
├── static/              # Archivos estáticos
├── Cargo.toml           # Dependencias y configuración de Rust
└── README.md           # Documentación
```

## 🎯 Características Detalladas

### Sistema de Temas
- Toggle intuitivo entre tema claro y oscuro
- Persistencia de preferencias usando LocalStorage
- Detección automática del tema del sistema
- Transiciones suaves entre temas
- Diseño consistente en ambos modos

### Arquitectura Backend
- Servidor web asíncrono de alto rendimiento
- Manejo eficiente de plantillas con Tera
- Rutas optimizadas para servir contenido estático
- Implementación thread-safe para concurrencia
- Gestión de estado del servidor en tiempo real

## 🤝 Contribución
Las contribuciones son bienvenidas. Por favor:
1. Haz fork del repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Realiza commit de tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia
Este proyecto está bajo la Licencia MIT.

## ✨ Autor
Mathias
