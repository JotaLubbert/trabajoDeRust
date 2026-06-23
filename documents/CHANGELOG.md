# 📋 Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

## [0.1.0] - 2024

### Added
- ✅ Conversión básica de imágenes a arte ASCII
- ✅ Soporte para múltiples formatos de imagen (PNG, JPG, BMP, etc.)
- ✅ Redimensionamiento personalizado de imagen
- ✅ Salida en blanco y negro
- ✅ Salida a color con True Color (24-bit)
- ✅ Interfaz CLI interactiva
- ✅ Validación de entrada de usuario
- ✅ Algoritmo de búsqueda binaria de brillo

### Technical
- Cálculo optimizado de brillo relativo (0.7R + 0.2G + 0.1B)
- Redimensionamiento de imagen con filtro Lanczos3
- Búsqueda binaria O(log n) para mapeo de caracteres
- Soporte para caracteres ASCII personalizados vía `CharBright.txt`

### Documentation
- README.md profesional
- Guía rápida (QUICKSTART.md)
- Documentación de arquitectura (ARCHITECTURE.md)
- Guía de contribución (CONTRIBUTING.md)

---

## Versiones Futuras Planeadas

### [0.2.0] - Planeado
- Manejo de errores mejorado (Result types)
- Salida a archivo (TXT, PNG)
- Múltiples paletas ASCII
- Configuración externa

### [0.3.0] - Planeado
- Procesamiento de video
- Paralelización con rayon
- Caché de imágenes
- Detección de capabilities de terminal

### [1.0.0] - Planeado
- API pública para usar como librería
- GUI básica
- Benchmarks completos
- Suite completa de tests

---

## Formato del Changelog

Seguimos [Keep a Changelog](https://keepachangelog.com/).

Categorías:
- **Added**: Nuevas funcionalidades
- **Changed**: Cambios en funcionalidad existente
- **Deprecated**: Funcionalidad que será removida
- **Removed**: Funcionalidad removida
- **Fixed**: Correcciones de bugs
- **Security**: Parches de seguridad
- **Technical**: Cambios técnicos/internos
- **Documentation**: Cambios en documentación

---

**Versión actual**: 0.1.0  
**Última actualización**: 2024
