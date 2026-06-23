# 🏗️ Arquitectura del Proyecto

Documentación técnica detallada de la arquitectura y diseño del sistema.

## Diagrama General

```
Usuario (CLI)
    ↓
main.rs (Input Management)
    ↓
custom_img.rs (Image Processing)
    ↓ (requiere)
read_files.rs (Load ASCII Mapping) → resources/CharBright.txt
    ↓
search_types.rs (Brightness Matching)
    ↓
Terminal Output (ASCII Art)
```

## Flujo de Datos

### 1. Inicialización

```rust
// 1. Carga del mapping ASCII
let (ascii, aparent_bright) = listed_file_content(char_bright);
// ascii: Vec<char>              ['.', ',', ':', ';', ...]
// aparent_bright: Vec<f64>      [0.0, 10.5, 20.3, 30.7, ...]

// 2. Obtención de entrada del usuario
let (img_path, x, y, color) = get_user_input();
```

### 2. Procesamiento de Imagen

```rust
// 1. Carga y redimensionamiento
let images = custom_img::open_resize_img(img_path, x, y_compressed);
// Resultado: ImageBuffer<Rgb<u8>, Vec<u8>>

// 2. Extracción de píxeles RGB
let mut pix: Vec<[u8; 3]> = vec![];
for i in images.pixels() {
    pix.push([i[0], i[1], i[2]]);
}

// 3. Creación de estructura CustomImg
let mut cus_img = CustomImg::new(x, y_compressed, pix);
```

### 3. Conversión RGB → ASCII

```rust
// 1. Cálculo de brillo relativo para cada píxel
fn relative_Brightness(&mut self) -> Vec<f64> {
    // Para cada píxel RGB
    brillo = 0.7 * R + 0.2 * G + 0.1 * B
}

// 2. Mapeo a caracteres ASCII
fn rgb_to_ascii(&mut self, ascii: Vec<char>, bright_of_char: Vec<f64>) {
    for brillo_pixel in rel_bright {
        // Busca el carácter con brillo más cercano
        idx = brillo_cercano(&bright_of_char, brillo_pixel);
        ascii_pixel.push(ascii[idx]);
    }
}
```

### 4. Renderizado

```rust
// Blanco y negro
pub fn display_ascii_art(&self) {
    // Itera sobre ascii_pixel
    // Agrega saltos de línea cada 'ancho' caracteres
}

// A color (requiere terminal soportada)
pub fn display_color_ascii_art(&self) {
    // Mismo layout pero cada char usa su color RGB original
    // Usa crate 'colored' para true color (24-bit)
}
```

## Diseño de Componentes

### CustomImg Struct

```rust
pub struct CustomImg {
    horizontal: u32,        // Ancho en caracteres (píxeles redimensionados)
    vertical: u32,          // Alto en caracteres (píxeles redimensionados)
    pixel: Vec<[u8; 3]>,    // Datos RGB de la imagen redimensionada
    ascii_pixel: Vec<char>, // Resultado ASCII después de conversión
}
```

**Invariantes**:
- `pixel.len() == horizontal * vertical`
- `ascii_pixel.len() == pixel.len()` (después de `rgb_to_ascii()`)
- Cada elemento de `pixel` es un array RGB válido [0-255, 0-255, 0-255]

### Algoritmo de Búsqueda de Brillo

**Función**: `brillo_cercano(&[f64], f64) -> usize`

```rust
pub fn brillo_cercano(lista_brillos: &[f64], objetivo: f64) -> usize {
    // Asume lista_brillos está ORDENADA (invariante crítico)
    
    let idx = lista_brillos.partition_point(|&b| b < objetivo);
    // partition_point retorna el índice donde sería insertado 'objetivo'
    
    // Casos límite
    if idx == 0 { return 0; }
    if idx == lista_brillos.len() { return lista_brillos.len() - 1; }
    
    // Compara distancias
    let anterior = lista_brillos[idx - 1];
    let siguiente = lista_brillos[idx];
    
    if (objetivo - anterior).abs() <= (siguiente - objetivo).abs() {
        idx - 1  // El anterior está más cerca
    } else {
        idx      // El siguiente está más cerca
    }
}
```

**Complejidad**: O(log n) - Búsqueda binaria

**Ejemplo**:
```
lista_brillos = [0.0, 10.5, 20.3, 30.7, 40.0]
objetivo = 15.0

partition_point devuelve idx=2 (donde 15.0 sería insertado)
anterior = 10.5, siguiente = 20.3
|15.0 - 10.5| = 4.5
|20.3 - 15.0| = 5.3
→ 4.5 < 5.3, retorna idx-1 = 1 (carácter con brillo 10.5)
```

## Flujo de Control del Usuario

```
┌─────────────────────────────────────────┐
│     INICIO - Mostrar prompts            │
└────────────────┬────────────────────────┘
                 │
    ┌────────────▼─────────────┐
    │  read_line(prompt)       │
    │  ↓ Input: imagen.jpg     │
    │  Valida: ¿existe?        │
    └────────────┬──────────────┘
                 │
    ┌────────────▼─────────────────────┐
    │  read_dimension(prompt)          │
    │  ↓ Input: 100                    │
    │  Valida: n > 0                   │
    │  Loop hasta entrada válida       │
    └────────────┬──────────────────────┘
                 │
    ┌────────────▼─────────────────────┐
    │  read_si_no(prompt)              │
    │  ↓ Input: s/n                    │
    │  Convierte a bool                │
    │  Loop hasta entrada válida       │
    └────────────┬──────────────────────┘
                 │
    ┌────────────▼───────────────────────┐
    │  Procesa imagen y genera ASCII Art │
    │  ↓ Renderiza resultado             │
    └────────────┬───────────────────────┘
                 │
         ┌───────▼────────┐
         │   FIN - Exit   │
         └────────────────┘
```

## Manejo de Errores

### Errores Críticos (Panic)
Estos causan pánico y terminan el programa:

```rust
// En read_files.rs
.parse::<f64>().unwrap()  // Si CharBright.txt tiene formato inválido
io::stdin().read_line(&mut input).expect(...)  // Si falla lectura de stdin

// En custom_img.rs
open(img_path).unwrap()   // Si imagen no existe o formato inválido
```

### Validación de Usuario

```rust
// read_dimension: Valida loop
fn read_dimension(prompt: &str) -> u32 {
    loop {
        match input.parse::<u32>() {
            Ok(n) if n > 0 => return n,  // ✓ Válido
            _ => println!("Por favor.."), // ✗ Reintentar
        }
    }
}

// read_si_no: Valida respuestas
match input.to_lowercase().as_str() {
    "s" | "si" | "sí" => true,
    "n" | "no" => false,
    _ => println!("Por favor.."),  // ✗ Reintentar
}
```

## Optimizaciones

### 1. Compresión Vertical
```rust
let y_compressed = y >> 1;  // Divide por 2 usando bit shift
// Por qué: Los caracteres son ~2x más altos que anchos
// Compensa la relación de aspecto terminal
```

### 2. Búsqueda Binaria
```rust
brillo_cercano() usa partition_point()
// O(log n) vs O(n) en búsqueda lineal
// Para 256 caracteres: ~8 comparaciones vs 256
```

### 3. Preallocación de String
```rust
let mut salida = String::with_capacity(
    self.ascii_pixel.len() + self.ascii_pixel.len() / ancho
);
// Evita realocaciones durante la construcción del string
```

### 4. Resizing de Imagen
```rust
img.resize_exact(x, y, FilterType::Lanczos3)
// Lanczos3: Alta calidad, buen equilibrio
// Alternativas: Nearest, Linear, CatmullRom, Gaussian
```

## Dependencias y Su Rol

### image crate (0.25.10)
```
Módulos usados:
├── open()                    → Carga imagen de disco
├── ImageBuffer               → Estructura de píxeles
├── imageops::FilterType::    → Algoritmos de resizing
│   └── Lanczos3
└── into_rgb8()              → Convierte a formato RGB de 8 bits
```

### colored crate (3.1.1)
```
Módulos usados:
└── Colorize trait
    └── .truecolor(r, g, b)  → Colorización 24-bit del texto
```

## Casos Límite Identificados

| Caso | Comportamiento | Recomendación |
|------|---|---|
| Imagen muy pequeña (1x1) | Genera 1 carácter | Usar dimensiones mín. 10x5 |
| Imagen muy grande (10000x10000) | Consumo alto de RAM/CPU | Limitar a 400x200 |
| Imagen corrupta | Panic en `open_resize_img()` | Validar formato previo |
| CharBright.txt no existe | Panic en `listed_file_content()` | Incluir en bundle |
| Terminal sin color 24-bit | Color ignorado gracefully | Detectar capabilities |

## Potenciales Mejoras

1. **Manejo de errores mejorado**: Result types en lugar de unwrap()
2. **Configuración externa**: Archivo de config para mapping de brillo personalizado
3. **Múltiples paletas**: Diferentes CharBright.txt según estilo
4. **Salida a archivo**: Guardar ASCII art en TXT o imagen
5. **Animaciones**: Procesar video frame por frame
6. **Paralelización**: rayon para procesar píxeles en paralelo
7. **Caché de imágenes**: Evitar reprocesamiento
8. **Interfaz gráfica**: GUI con preview en tiempo real

---

Para más información, consulta [README.md](../README.md) o los comentarios en el código fuente.
