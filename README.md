# ASCII Art Image Converter

Un conversor de imágenes a arte ASCII escrito en Rust. Transforma cualquier imagen en representación textual usando caracteres ASCII, con soporte para visualización a color.

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active-brightgreen?style=flat-square)

</div>

## Tabla de Contenidos

- [Características](#características)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Uso](#uso)
- [Estructura del Proyecto](#estructura-del-proyecto)
- [Cómo Funciona](#cómo-funciona)
- [Dependencias](#dependencias)
- [Ejemplos](#ejemplos)

## Características

- ✅ Conversión de imágenes a arte ASCII
- ✅ Soporte para múltiples formatos de imagen (PNG, JPG, BMP, etc.)
- ✅ Redimensionamiento personalizado
- ✅ Salida en blanco y negro o **a color**
- ✅ Interfaz interactiva
- ✅ Mapping inteligente de brillo con caracteres ASCII

## Requisitos

- **Rust 1.70 o superior** ([Descargar](https://www.rust-lang.org/tools/install))
- **Cargo** (incluido con Rust)

### Verificar instalación:
```bash
rustc --version
cargo --version
```

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/usuario/trabajoDeRust.git
cd trabajoDeRust
```

### 2. Compilar el proyecto

**Modo Debug** (compilación rápida, menos optimizaciones):
```bash
cargo build
```

**Modo Release** (compilación optimizada para producción):
```bash
cargo build --release
```

Las compilaciones estarán disponibles en:
- Debug: `./target/debug/trabajoDeRust`
- Release: `./target/release/trabajoDeRust`

## Uso

### Ejecutar con Cargo

```bash
cargo run
```

### Ejecutar el binario compilado

```bash
# Modo Debug
./target/debug/trabajoDeRust

# Modo Release (más rápido)
./target/release/trabajoDeRust
```

### Pasos de Uso Interactivo

Al ejecutar la aplicación, se te pedirá:

1. **Ruta de la imagen**: Proporciona la ruta absoluta o relativa a tu imagen
   ```
   Ruta de la imagen: /ruta/a/imagen.jpg
   ```

2. **Ancho de salida**: Número de caracteres de ancho
   ```
   Ancho de salida: 120
   ```

3. **Alto de salida**: Número de caracteres de alto
   ```
   Alto de salida: 40
   ```

4. **Color (s/n)**: Salida a color o en blanco y negro
   ```
   ¿Quieres el resultado a color? (s/n): s
   ```

El arte ASCII se mostrará en la terminal con los parámetros especificados.

## Estructura del Proyecto

```
trabajoDeRust/
├── Cargo.toml                 # Configuración del proyecto y dependencias
├── README.md                  # Este archivo
├── resources/
│   └── CharBright.txt        # Mapping de caracteres ASCII con valores de brillo
├── src/
│   ├── main.rs               # Punto de entrada y interfaz CLI
│   ├── custom_img.rs         # Estructura CustomImg y lógica de conversión
│   ├── read_files.rs         # Lectura y parseo del archivo CharBright.txt
│   └── search_types.rs       # Algoritmo de búsqueda de brillo cercano
└── target/                   # Directorio de compilación (generado)
    ├── debug/                # Binarios en modo debug
    └── release/              # Binarios optimizados
```

## Cómo Funciona

### Algoritmo General

1. **Carga de mapping**: Lee `CharBright.txt` que contiene pares de caracteres ASCII y sus valores de brillo
2. **Carga de imagen**: Abre la imagen usando la crate `image`
3. **Redimensionamiento**: Redimensiona la imagen a las dimensiones especificadas usando filtro Lanczos3
4. **Conversión RGB a ASCII**:
   - Calcula el brillo relativo de cada píxel: `0.7*R + 0.2*G + 0.1*B`
   - Encuentra el carácter ASCII con brillo más cercano
   - Almacena el carácter correspondiente
5. **Visualización**: Muestra el resultado en blanco y negro o con color RGB original

### Módulos

#### `main.rs`
- **Punto de entrada** del programa
- **Interfaz interactiva** para obtener parámetros del usuario
- **Validación de entrada**: asegura que dimensiones sean positivas
- Funciones auxiliares:
  - `read_line()`: Lee input de usuario
  - `read_dimension()`: Valida entrada numérica
  - `read_si_no()`: Valida entrada booleana
  - `get_user_input()`: Recolecta todos los parámetros

#### `custom_img.rs`
**Estructura `CustomImg`**:
- `horizontal`, `vertical`: Dimensiones de la imagen
- `pixel`: Vector de píxeles RGB
- `ascii_pixel`: Vector de caracteres ASCII resultantes

**Métodos principales**:
- `new()`: Crea una nueva instancia
- `relative_Brightness()`: Calcula brillo relativo de cada píxel
- `rgb_to_ascii()`: Mapea píxeles a caracteres ASCII
- `display_ascii_art()`: Imprime resultado en blanco y negro
- `display_color_ascii_art()`: Imprime resultado con colores RGB originales

**Función `open_resize_img()`**:
- Abre imagen desde ruta
- Redimensiona usando filtro Lanczos3 (alta calidad)
- Retorna `ImageBuffer` en formato RGB8

#### `read_files.rs`
**Función `listed_file_content()`**:
- Parsea `CharBright.txt`
- Retorna tupla: `(Vec<char>, Vec<f64>)`
- Cada línea: `caracter valor_brillo` (ej: `. 0.1`)

#### `search_types.rs`
**Función `brillo_cercano()`**:
- Búsqueda binaria eficiente
- Encuentra carácter con brillo más próximo al objetivo
- Usa `partition_point()` para O(log n) complexity
- Retorna índice del carácter más cercano

### Cálculo de Brillo

La fórmula de brillo relativo percibido (luminancia):
```
Brillo = 0.7 × R + 0.2 × G + 0.1 × B
```

Esto refleja la sensibilidad diferencial del ojo humano a los colores:
- **Rojo** (70%): Máxima sensibilidad
- **Verde** (20%): Sensibilidad media
- **Azul** (10%): Mínima sensibilidad

## Dependencias

| Crate | Versión | Uso |
|-------|---------|-----|
| `image` | 0.25.10 | Carga, redimensiona y procesa imágenes |
| `colored` | 3.1.1 | Colorización de texto en terminal |

Instáladas automáticamente por Cargo.

## 📝 Archivo CharBright.txt

Mapeo de caracteres ASCII a valores de brillo (0.0-255.0).

Formato de ejemplo:
```
. 0.0
, 10.5
: 20.3
; 30.7
...
M 255.0
```

- Caracteres con **bajo brillo** (puntos, comas) → áreas oscuras
- Caracteres con **alto brillo** (M, W) → áreas claras

## Ejemplos de Uso

### Ejemplo 1: Conversión Simple

```bash
$ cargo run
Ruta de la imagen: /home/usuario/foto.jpg
Ancho de salida: 80
Alto de salida: 24
¿Quieres el resultado a color? (s/n): n

# Resultado en blanco y negro:
.......................................... ....................
:::::::::::::::::::  ::;;;;;;;;;;;;;;;;;::...
```

### Ejemplo 2: Salida a Color

```bash
cargo run
# ... (mismos parámetros)
¿Quieres el resultado a color? (s/n): s
# Se mostrará el arte ASCII coloreado con los RGB originales
```

## Casos de Uso

- Convertir fotos a arte ASCII
- Crear animaciones ASCII frame por frame
- Generar representaciones de imágenes para documentos de texto
- Arte terminal interactivo
- Aprendizaje de procesamiento de imágenes

## 🔍 Notas Técnicas

- **Compresión vertical**: El programa comprime a la mitad el alto de la imagen internamente (`y_compressed = y >> 1`) para compensar la altura de los caracteres en la terminal
- **Búsqueda binaria**: El módulo `search_types` usa `partition_point()` para O(log n) lookup
- **Validación**: Se validan todas las entradas de usuario (dimensiones positivas, respuestas s/n)
- **Manejo de errores**: Los errores críticos (archivo no encontrado, formato inválido) causan pánico

## Compilación y Ejecución Detallada

### Limpiar compilaciones previas:
```bash
cargo clean
```

### Compilar con características específicas:
```bash
cargo build --release --verbose
```

### Ejecutar con backtrace de errores:
```bash
RUST_BACKTRACE=1 cargo run
```

## Licencia

Este proyecto está bajo licencia MIT. Consulta [LICENSE](LICENSE) para más detalles.

---

