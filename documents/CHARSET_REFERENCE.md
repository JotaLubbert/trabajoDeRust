# 📚 Referencia del Archivo CharBright.txt

Documentación completa del formato de mapping ASCII-a-brillo.

## Descripción General

El archivo `resources/CharBright.txt` define el mapeo entre caracteres ASCII y sus valores de brillo percibido. Cada línea representa un carácter y su correspondiente nivel de luminancia.

## Formato

```
<carácter> <valor-brillo>
```

Donde:
- **carácter**: Un carácter ASCII imprimible (1 carácter)
- **espaciador**: Un espacio (` `)
- **valor-brillo**: Un número decimal 0.0-255.0 (representando luminancia)

### Ejemplo de líneas válidas

```
. 0.0
, 10.5
: 20.3
; 30.7
i 40.2
| 50.0
I 75.3
M 255.0
```

## Características del Brillo

- **0.0**: Negro puro (más oscuro)
- **127.5**: Gris medio
- **255.0**: Blanco puro (más claro)

### Orden Importante

**Las líneas DEBEN estar ordenadas por brillo ascendente.** El algoritmo de búsqueda utiliza `partition_point()` que asume un array ordenado.

```
✓ CORRECTO
. 0.0
, 10.5
: 20.3
M 255.0

✗ INCORRECTO (desordenado)
. 0.0
: 20.3
, 10.5
M 255.0
```

## Recomendaciones de Selectores

Para obtener mejor contraste visual:

| Tipo | Caracteres | Rango de Brillo |
|------|-----------|---|
| Muy oscuro | `.,-~` | 0-30 |
| Oscuro | `:;il!` | 30-70 |
| Medio | `x%o*` | 70-130 |
| Claro | `=+*` | 130-180 |
| Muy claro | `M@W` | 180-255 |

## Personalización

### Crear tu propio CharBright.txt

```bash
# 1. Crea un archivo nuevo
vi resources/CharBright.txt

# 2. Agrega líneas ordenadas por brillo
# Ejemplo minimalista (requiere análisis de brillo de cada char)
  0.0
. 40.0
: 80.0
o 120.0
@ 255.0

# 3. Asegúrate que esté ORDENADO ascendentemente
```

### Análisis de Brillo Percibido

Para calcular el brillo de un carácter (si quieres crear uno personalizado):

```
Brillo percibido = (anchura * altura * densidad_píxeles) × factor_luminoso
```

Para simplificar, los caracteres más "llenos" tienen mayor brillo:
- `.` - 1 píxel (muy bajo) → ~0-20
- `o` - 8 píxeles (medio) → ~80-120
- `@` - 16 píxeles (lleno) → ~200-255

## Consideraciones Especiales

### Caracteres Especiales

Algunos caracteres requieren escaping:
- **Espacio**: El espacio es un carácter válido para arte ASCII
  ```
    0.0
  ```
- **Tabulación**: Evitar (puede causar problemas de parseo)

### Caracteres Unicode

El archivo típicamente usa ASCII de 7 bits:
```
. , : ; i | I M
```

Soporte Unicode: Depende de la terminal y configuración de locale.

## Validación

Para validar tu archivo:

```bash
# 1. Verifica que esté ordenado
sort -k2 -n resources/CharBright.txt

# 2. Verifica línea por línea
cat -A resources/CharBright.txt

# 3. Ejecuta cargo run con debugging
RUST_BACKTRACE=1 cargo run
```

## Problemas Comunes

| Problema | Causa | Solución |
|----------|-------|----------|
| Panic al cargar | Archivo no encontrado | Verifica ruta `resources/CharBright.txt` |
| Caracteres invertidos | Orden incorrecto | Ordena ascendentemente por brillo |
| Muchos caracteres iguales | Resolución baja | Aumenta ancho/alto de salida |
| Resultado muy oscuro | Brillo inicial bajo | Comienza con `.` en ~0.0 |

## Ejemplos Predefinidos

### Paleta Estándar (Recomendada)
```
. 0.0
, 10.0
: 20.0
; 30.0
i 40.0
| 50.0
I 60.0
= 70.0
+ 80.0
* 90.0
x 100.0
o 110.0
X 120.0
O 130.0
% 140.0
@ 255.0
```

### Paleta Minimalista (3 niveles)
```
. 0.0
= 127.5
@ 255.0
```

### Paleta Detallada (20+ caracteres)
Ver archivo incluido en `resources/CharBright.txt`

## Formato Extendido (Futuro)

Versiones futuras podrían soportar:
```
# Comentarios
character brightness densidade_visual color_sugerido
```

Por ahora, mantente con el formato simple de 2 columnas.

---

**Nota**: La búsqueda de brillo cercano es robusta ante valores atípicos. El algoritmo siempre encontrará el carácter ASCII más cercano al brillo del píxel.
