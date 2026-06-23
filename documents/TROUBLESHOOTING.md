# 🐛 Guía de Solución de Problemas

Soluciones a problemas comunes al usar ASCII Art Image Converter.

## Instalación y Compilación

### Error: `rustc: command not found`

```
error: rustc: command not found
```

**Causa**: Rust no está instalado o no está en el PATH.

**Solución**:
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cargar configuración
source $HOME/.cargo/env

# Verificar
rustc --version
```

### Error: `Cargo.toml not found`

```
error: Cargo.toml not found
```

**Causa**: No estás en el directorio del proyecto.

**Solución**:
```bash
cd /ruta/a/trabajoDeRust
cargo build
```

### Error: `edition "2024" is not supported`

```
error: edition "2024" is not supported
```

**Causa**: Tu versión de Rust es muy antigua.

**Solución**:
```bash
# Actualizar Rust
rustup update

# Verificar versión
rustc --version  # Debe ser 1.75+
```

### Compilación muy lenta

**Causa**: Modo Debug tiene pocas optimizaciones.

**Soluciones**:
```bash
# Opción 1: Usar Release (más rápido en ejecución)
cargo build --release

# Opción 2: Usar compilación incremental
cargo build -j 4  # 4 hilos paralelos

# Opción 3: Limpiar y reconstruir
cargo clean
cargo build --release
```

## Ejecución

### Error: `No such file or directory`

```
Ruta de la imagen: /foto.jpg
Error al procesar imagen
```

**Causa**: La ruta de la imagen no existe.

**Soluciones**:
```bash
# Usar ruta absoluta completa
/home/usuario/imagenes/foto.jpg

# Usar ruta relativa desde el directorio del proyecto
./resources/mi-imagen.jpg

# Listar archivos
ls -la /ruta/a/archivos/
```

### Error: `panicked at 'called Result::unwrap() on an Err value'`

```
thread 'main' panicked at 'called Result::unwrap()...'
```

**Causa**: Formato de imagen no soportado o archivo corrupto.

**Solución**:
```bash
# Convertir imagen a formato soportado
convert imagen.gif imagen.jpg  # Usando ImageMagick
ffmpeg -i imagen.gif imagen.png  # Usando ffmpeg

# Soportados: PNG, JPG, BMP, TIFF, GIF, TGA
```

### Error: `panicked at 'attempt to read from stdin failed'`

**Causa**: Entrada estándar rota o desconectada.

**Solución**:
```bash
# Ejecutar con entrada conectada
cargo run < /dev/stdin

# Conectar archivo de entrada
cargo run < input.txt
```

## Problemas con Imágenes

### Imagen se ve muy pixelada o distorsionada

**Causa**: Dimensiones muy pequeñas.

**Soluciones**:
```
Ancho de salida: 40  ← Demasiado pequeño
Ancho de salida: 120  ← Mejor
Ancho de salida: 200  ← Muy detallado
```

**Recomendaciones** por tipo de imagen:
- Fotos: 120-160 ancho
- Logos: 80-100 ancho
- Arte detallado: 160-200 ancho

### Resultado en color se ve apagado

**Causa**: Terminal no soporta True Color (24-bit).

**Verificar soporte**:
```bash
# Probar color
echo -e '\e[38;2;255;0;0mRojo\e[0m'

# Si ves rojo, soporta True Color
# Si ves colores incorrectos, terminal limitada
```

**Soluciones**:
```bash
# Usar salida blanco y negro
¿Quieres el resultado a color? (s/n): n

# O cambiar terminal:
# GNOME Terminal, iTerm2, Konsole (soportan True Color)
```

### Imagen tarda mucho en procesarse

**Causa**: Dimensiones muy grandes.

**Soluciones**:
```
Ancho de salida: 400  ← Muy grande (lento)
Ancho de salida: 150  ← Óptimo

# Con compilación Release es mucho más rápido
cargo build --release
./target/release/trabajoDeRust
```

**Benchmarks estimados**:
- 80x24: < 100ms
- 120x40: ~200ms
- 160x50: ~400ms
- 200x60: ~700ms

## Problemas con CharBright.txt

### Error: `panicked at 'called Option::unwrap() on a None value'`

**Causa**: `CharBright.txt` tiene formato inválido.

**Verificar formato**:
```bash
# Debe tener líneas como: "c número"
cat resources/CharBright.txt

# Debe verse:
# . 0.0
# , 10.5
# etc.
```

### Resultado con caracteres incorrectos

**Causa**: Orden incorrecto en CharBright.txt.

**Verificar orden**:
```bash
# Los números DEBEN estar ordenados ascendentemente
sort -k2 -n resources/CharBright.txt
```

**Si no está ordenado**:
```bash
# Crear version ordenada
sort -k2 -n resources/CharBright.txt > resources/CharBright_sorted.txt
mv resources/CharBright_sorted.txt resources/CharBright.txt
```

## Problemas del Sistema

### Error: `Segmentation fault`

**Causa poco probable**: Posible corrupción de memoria.

**Soluciones**:
```bash
# Limpiar y reconstruir
cargo clean
cargo build --release

# Ejecutar con backtrace
RUST_BACKTRACE=full cargo run
```

### Error: `Out of memory`

**Causa**: Imagen demasiado grande.

**Soluciones**:
```bash
# Reducir dimensiones
Ancho de salida: 100  ← Más pequeño

# O reducir tamaño de imagen primero
convert imagen.jpg -resize 1024x768 imagen-pequeña.jpg
```

### Problema: Caracteres unicode se muestran mal

**Causa**: Locale o terminal no soporta UTF-8.

**Soluciones**:
```bash
# Verificar locale
locale

# Establecer UTF-8
export LANG=es_ES.UTF-8
export LC_ALL=es_ES.UTF-8

# Luego ejecutar
cargo run
```

## Ayuda Avanzada

### Habilitar Backtrace Completo

```bash
# Backtrace completo
RUST_BACKTRACE=full cargo run

# Backtrace sin optimizaciones
RUST_BACKTRACE=1 cargo build --release
```

### Debugging con GDB

```bash
# Compilar con símbolos
cargo build

# Ejecutar con gdb
gdb --args ./target/debug/trabajoDeRust

# Dentro de gdb:
(gdb) run
(gdb) bt  # Backtrace
(gdb) print variable_name
```

### Verificar Dependencias

```bash
# Listar dependencias instaladas
cargo tree

# Verificar actualización de dependencias
cargo outdated

# Actualizar dependencias
cargo update
```

## Contacto y Reporte de Bugs

Si no encuentras la solución aquí:

1. **Verifica**: Que Rust esté actualizado (`rustup update`)
2. **Busca**: En issues existentes de GitHub
3. **Describe**: El problema exacto, SO, versiones, pasos para reproducir
4. **Incluye**: Output completo, archivo de configuración, imagen de prueba
5. **Abre**: GitHub Issue con todos los detalles

## Checklist de Solución de Problemas

- [ ] ¿Rust está actualizado? (`rustup update`)
- [ ] ¿Estoy en el directorio correcto?
- [ ] ¿La ruta de la imagen es correcta?
- [ ] ¿El archivo existe? (`ls archivo`)
- [ ] ¿Formato de imagen es válido? (PNG, JPG, BMP)
- [ ] ¿CharBright.txt está en recursos?
- [ ] ¿Imagen no es demasiado grande?
- [ ] ¿Terminal soporta colores? (para salida color)
- [ ] ¿Cargo clean y rebuild? (problemas persistentes)
- [ ] ¿Compilaste con Release para mejor rendimiento?

---

**¿Aún no funciona?** Abre un issue en GitHub con detalles completos. 🐛
