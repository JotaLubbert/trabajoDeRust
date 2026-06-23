# 🤝 Guía de Contribución

¡Gracias por tu interés en contribuir a ASCII Art Image Converter!

## Código de Conducta

Sé respetuoso y constructivo. Todos merecen un ambiente seguro y acogedor.

## Cómo Contribuir

### 1. Reportar Bugs

**Antes de reportar**, verifica que el bug no haya sido reportado.

**Al reportar, incluye**:
- Versión de Rust (`rustc --version`)
- Sistema operativo
- Pasos para reproducir
- Comportamiento esperado vs actual
- Screenshot o output si aplica

```
Título: [BUG] La imagen PNG no se carga correctamente

Versión: Rust 1.75.0
SO: Linux Ubuntu 22.04

Pasos:
1. Ejecutar cargo run
2. Ingresar ruta a imagen PNG
3. Resultado: Error

Comportamiento esperado: Debería cargar la imagen
Comportamiento actual: Se genera panic
```

### 2. Sugerir Mejoras

Abre un issue describiendo:
- Problema que resuelve
- Solución propuesta
- Beneficios
- Posibles alternativas

### 3. Enviar Pull Requests

#### Setup

```bash
# Fork el repositorio en GitHub
git clone https://github.com/tu-usuario/trabajoDeRust.git
cd trabajoDeRust
git checkout -b feature/tu-nombre-descriptivo
```

#### Desarrollo

1. **Código limpio**:
   ```bash
   # Formatea código
   cargo fmt
   
   # Verifica estilo
   cargo clippy
   
   # Ejecuta tests
   cargo test
   ```

2. **Commits claros**:
   ```bash
   git commit -m "feat: agregar soporte para paletas personalizadas"
   git commit -m "fix: corregir overflow en búsqueda binaria"
   git commit -m "docs: actualizar guía de instalación"
   ```

   **Prefijos comunes**:
   - `feat:` - Nueva funcionalidad
   - `fix:` - Corrección de bug
   - `docs:` - Documentación
   - `style:` - Cambios de formato (no afectan lógica)
   - `refactor:` - Refactorización
   - `perf:` - Mejoras de rendimiento
   - `test:` - Tests

3. **Push y PR**:
   ```bash
   git push origin feature/tu-nombre-descriptivo
   # Abre PR en GitHub
   ```

#### Descripción del PR

```markdown
## Descripción
Breve descripción de los cambios

## Tipo de Cambio
- [ ] Bug fix
- [ ] Nueva feature
- [ ] Mejora de documentación

## Cambios
- Cambio 1
- Cambio 2

## Testing
Describe cómo testeaste los cambios

## Screenshots (si aplica)
Adjunta imágenes si hay cambios visuales
```

## Estándares de Código

### Rust Style Guide

```rust
// ✓ BIEN
fn calculate_brightness(rgb: [u8; 3]) -> f64 {
    0.7 * rgb[0] as f64 + 0.2 * rgb[1] as f64 + 0.1 * rgb[2] as f64
}

// ✗ MAL
fn calc_bright(r: u8,g:u8,b:u8)->f64{
    0.7*r as f64+0.2*g as f64+0.1*b as f64
}
```

### Documentación

```rust
/// Convierte píxeles RGB a arte ASCII
/// 
/// # Arguments
/// * `ascii` - Vector de caracteres ASCII
/// * `bright_of_char` - Vector de valores de brillo
///
/// # Example
/// ```
/// cus_img.rgb_to_ascii(vec!['a', 'b'], vec![0.0, 1.0]);
/// ```
pub fn rgb_to_ascii(&mut self, ascii: Vec<char>, bright_of_char: Vec<f64>) {
    // ...
}
```

### Manejo de Errores

**Preferir**:
```rust
// Use Result types
pub fn open_image(path: &str) -> Result<Image, ImageError> {
    // ...
}
```

**Evitar**:
```rust
// No use unwrap() sin justificación
let img = open(path).unwrap();
```

## Estructura de Archivos

Al agregar nuevos módulos:

```
src/
├── main.rs              (no modificar estructura)
├── lib.rs              (si aplica)
├── feature/
│   ├── mod.rs          (público, expone API)
│   ├── implementation.rs (privado, implementación)
│   └── tests.rs        (tests unitarios)
```

## Testing

```bash
# Ejecutar todos los tests
cargo test

# Test específico
cargo test test_brillo_cercano

# Con output
cargo test -- --nocapture

# Coverage (si tienes tarpaulin)
cargo tarpaulin --out Html
```

### Escribir Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brillo_cercano_encuentra_valor_exacto() {
        let lista = vec![0.0, 10.0, 20.0];
        assert_eq!(brillo_cercano(&lista, 10.0), 1);
    }

    #[test]
    #[should_panic]
    fn test_imagen_invalida_panics() {
        open_resize_img("no-existe.jpg", 100, 100);
    }
}
```

## Proceso de Review

1. GitHub Actions ejecuta `cargo test` y `cargo clippy`
2. Reviewer revisa código
3. Se solicitan cambios si es necesario
4. Aprobación y merge

## Preguntas Frecuentes

**P: ¿Necesito firmar un CLA?**
R: No para este proyecto (educativo).

**P: ¿Cuál es la política de versionado?**
R: Semver: MAJOR.MINOR.PATCH

**P: ¿A qué rama hago push?**
R: A una rama descriptiva basada en `main`

**P: ¿Cuánto tarda revisar un PR?**
R: 1-3 días típicamente

## Recursos

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Conventional Commits](https://www.conventionalcommits.org/)

## Contacto

- Issues: GitHub Issues
- Email: [contacto]

¡Gracias por contribuir! 🎉
