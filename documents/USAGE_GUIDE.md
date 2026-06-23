# 🎬 Manual de Uso Visual

Guía paso a paso con explicaciones visuales.

## 📥 Fase 1: Instalación

```bash
# Paso 1: Clonar el repositorio
$ git clone https://github.com/usuario/trabajoDeRust.git
$ cd trabajoDeRust

# Paso 2: Compilar (primera vez puede tardar 2-3 minutos)
$ cargo build --release

# Verificación: ¿Se compiló sin errores?
✓ Compiled successfully

# Resultado: ejecutable creado
$ ls -la target/release/trabajoDeRust
-rwxr-xr-x ... trabajoDeRust
```

## 🚀 Fase 2: Ejecución

### Opción A: Usar `cargo run`

```bash
$ cargo run

Ruta de la imagen: /ruta/a/imagen.jpg
Ancho de salida: 120
Alto de salida: 40
¿Quieres el resultado a color? (s/n): s

[Aquí aparecerá el arte ASCII coloreado]
```

### Opción B: Ejecutar binario directamente

```bash
$ ./target/release/trabajoDeRust

[Mismo flujo que arriba]
```

## 📊 Fase 3: Entradas Interactivas

### Entrada 1: Ruta de la Imagen

```
┌─────────────────────────────────┐
│ Ruta de la imagen:              │
└─────────────────────────────────┘

Ejemplos de respuestas válidas:
  • /home/usuario/fotos/vacaciones.jpg
  • ./recursos/logo.png
  • C:\Users\usuario\imagen.bmp
  • ./CharBright.txt (error - no es imagen)
```

### Entrada 2: Ancho de Salida

```
┌─────────────────────────────────┐
│ Ancho de salida:                │
└─────────────────────────────────┘

Rango: 10-400 caracteres
Recomendación por dispositivo:
  • Terminal pequeña (80 cols):      80
  • Terminal normal (120 cols):      100
  • Monitor ancho (150+ cols):       160
  • Presentación (proyector):        200
```

### Entrada 3: Alto de Salida

```
┌─────────────────────────────────┐
│ Alto de salida:                 │
└─────────────────────────────────┘

Rango: 5-200 caracteres
Consejo: El programa comprime el alto automáticamente
         por la relación de aspecto de caracteres
```

### Entrada 4: Color Sí/No

```
┌─────────────────────────────────────────┐
│ ¿Quieres el resultado a color? (s/n):   │
└─────────────────────────────────────────┘

Respuestas válidas:
  • s / sí / sí        → Salida a color
  • n / no             → Blanco y negro
  • cualquier otra     → Pregunta de nuevo
```

## 🎨 Fase 4: Salida (Ejemplos)

### Salida Blanco y Negro

```
............................
.::;;;;;;;;;;;;;;;;;..........
:::::;;;;;;;;;;;;;;::........
;;;;;;;;;:::::::::;;:........
;;;;;;;;;       ;;;;:........
;;;;;;             :;:......
;;;;;;             ::;;;;...
;;;;;;;;;;     ;;;;;;::::...
;;;;;;;;;;;;;;;;;;;;;;::....
.............;;;;;;;;;......
............................
```

### Salida a Color

```
[La misma estructura pero cada carácter
con el color RGB del píxel original]

Ejemplo conceptual:
  🔴 caracteres rojos     (píxeles rojos en imagen)
  🟢 caracteres verdes    (píxeles verdes en imagen)
  🔵 caracteres azules    (píxeles azules en imagen)
  ⚪ caracteres blancos   (píxeles brillantes)
  ⚫ caracteres negros    (píxeles oscuros)
```

## 📐 Fase 5: Entrada vs Salida

### Ejemplo Real

**Entrada**:
- Imagen: `foto_de_gato.jpg` (1920×1080 pixels originales)
- Ancho: 100 caracteres
- Alto: 30 caracteres
- Color: sí

**Procesamiento**:
```
Original (1920×1080)
    ↓ Redimensionar
ASCII (100×15) [nota: 30>>1 = 15]
    ↓ Convertir a ASCII
Salida (100 caracteres × 30 líneas)
    con colores RGB
```

**Salida**:
```
[Arte ASCII de 100 chars de ancho × 30 líneas]
[Cada carácter coloreado según píxel original]
```

## 💾 Fase 6: Guardar Salida

### Opción A: Copiar de terminal

```bash
# En terminal Linux/Mac, usar:
# 1. Selecciona el área con ratón
# 2. Ctrl+C
# 3. Ctrl+V en editor
```

### Opción B: Redirigir a archivo (futuro)

```bash
# Ahora no soportado, pero planeado para v0.2
cargo run > output.txt 2>&1
```

### Opción C: Screenshot

```bash
# Tomar screenshot de terminal
# (funciona para salida a color)

# Linux:
gnome-screenshot

# Mac:
cmd + shift + 4
```

## ⚡ Fase 7: Rendimiento

### Benchmarks Típicos

| Entrada | Procesamiento | Salida |
|---------|---|---|
| 80×24 | <100ms | Instantáneo |
| 120×40 | ~200ms | <1s visualización |
| 160×50 | ~400ms | 1-2s visualización |
| 200×60 | ~700ms | 2-3s visualización |

**En modo Release** (recomendado):
```bash
cargo build --release
./target/release/trabajoDeRust
```

## 🔄 Flujo Completo Resumido

```
START
  ↓
[Solicitar ruta imagen]
  ↓
[Solicitar ancho]
  ↓
[Solicitar alto]
  ↓
[Solicitar color sí/no]
  ↓
[Cargar imagen]
  ↓
[Redimensionar]
  ↓
[Convertir RGB → ASCII]
  ↓
[Mostrar en terminal]
  ↓
END
```

## 🎯 Casos de Uso Comunes

### Caso 1: Preview de Foto

```bash
Ruta: /home/usuario/IMG_001.jpg
Ancho: 100
Alto: 30
Color: sí
→ Muestra foto como arte ASCII coloreado
```

### Caso 2: Gráfico para Documento

```bash
Ruta: ./logo.png
Ancho: 80
Alto: 20
Color: no
→ Salida blanco y negro para pegar en doc
```

### Caso 3: Arte ASCII Detallado

```bash
Ruta: ./retrato.jpg
Ancho: 200
Alto: 60
Color: sí
→ Resultado muy detallado y preciso
```

## ⚠️ Cosas a Evitar

```
❌ EVITAR                    ✅ HACER
─────────────────────────────────────────
Ancho: 5                     Ancho: ≥20
Alto: 2                      Alto: ≥10

Imagen corrupta              Convertir a PNG
Archivo .gif                 primero

Terminal muy pequeña         Aumentar ventana
(< 80 caracteres)           o reducir ancho

Imagen de 100 MB             Comprimir primero
```

## 🆘 Validaciones

El programa valida automáticamente:

```
✓ Ancho > 0
✓ Alto > 0
✓ Imagen existe
✓ Respuesta s/n válida
✓ Formato de imagen válido
✓ CharBright.txt presente
✓ Suficiente memoria

✗ Si algo falla → Solicita reintento
```

## 📱 Adaptación Según Dispositivo

### En Laptop

```bash
Ruta: /foto.jpg
Ancho: 120        ← Pantalla típica ~120 cols
Alto: 40          ← Espacio vertical típico
Color: sí         ← Laptop moderno soporta color
```

### En Terminal Pequeña

```bash
Ruta: /foto.jpg
Ancho: 80         ← Límite típico
Alto: 24          ← Terminal standar
Color: no         ← Terminal antigua puede no soportar
```

### En Monitor 4K / Proyector

```bash
Ruta: /foto.jpg
Ancho: 240        ← Mucho espacio
Alto: 60          ← Bastante altura
Color: sí         ← Máxima calidad
```

## 📞 Diferencia entre Errores

### Error: Imagen No Encontrada

```
¿Qué pasó?
  Ruta: /ruta/inexistente.jpg
  → Archivo no existe en esa ruta

Qué hacer:
  1. Verifica la ruta correcta
  2. Usa ruta absoluta
  3. Usa pwd para verificar directorio
```

### Error: Archivo Corrupto

```
¿Qué pasó?
  Archivo existe pero está corrupto
  → Formato no es imagen válida

Qué hacer:
  1. Convierte a PNG o JPG
  2. Valida con `file imagen.jpg`
  3. Abre en visor de imágenes
```

---

**Guía Visual Completada** ✓

Para más detalles técnicos: [ARCHITECTURE.md](ARCHITECTURE.md)
Para solución de problemas: [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
Volver al [README.md](../README.md)
