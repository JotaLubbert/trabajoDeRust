# trabajoDeRust

Un proyecto en Rust para procesar y manipular imágenes.

## Requisitos

- Rust 1.70+ ([Descargar](https://www.rust-lang.org/))

## Instalación y Compilación

1. **Clonar el repositorio:**
```bash
git clone <URL-del-repo>
cd trabajoDeRust
```

2. **Compilar:**
```bash
cargo build
```

O compilar optimizado para producción:
```bash
cargo build --release
```

## Ejecutar

```bash
cargo run
```

O directamente el binario compilado:
```bash
./target/debug/trabajoDeRust          # Debug
./target/release/trabajoDeRust        # Release
```

## Estructura del Proyecto

- `src/main.rs` - Punto de entrada
- `src/custom_img.rs` - Módulo de procesamiento de imágenes
- `src/read_file.rs` - Lectura de archivos
- `src/search_types.rs` - Búsqueda de tipos
- `resources/` - Archivos de recursos

## Dependencias

- `colored` - Para colores en la terminal
- `image` - Para procesamiento de imágenes