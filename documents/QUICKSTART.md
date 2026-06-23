# ⚡ Guía Rápida

Comienza en 2 minutos.

## Instalación Rápida

```bash
git clone https://github.com/usuario/trabajoDeRust.git
cd trabajoDeRust
cargo build --release
./target/release/trabajoDeRust
```

## Primera Ejecución

```
Ruta de la imagen: /ruta/a/tu/imagen.jpg
Ancho de salida: 100
Alto de salida: 50
¿Quieres el resultado a color? (s/n): s
```

¡Listo! Tu arte ASCII se mostrará en la terminal.

## Ejemplos de Rutas

- **Ruta absoluta**: `/home/usuario/imagenes/foto.jpg`
- **Ruta relativa**: `./resources/imagen.jpg`
- **Windows**: `C:\Usuarios\usuario\imagen.jpg`

## Recomendaciones

| Caso | Ancho | Alto | Color |
|------|-------|------|-------|
| Vista rápida | 80 | 24 | No |
| Presentación | 120 | 40 | Sí |
| Alta definición | 200 | 60 | Sí |
| Arte detallado | 160 | 50 | No |

## Troubleshooting Rápido

| Problema | Solución |
|----------|----------|
| `No such file or directory` | Verifica la ruta de la imagen |
| Imagen se ve distorsionada | Aumenta el ancho/alto |
| Compilación lenta | Usa `cargo build --release` |

¡Para más detalles, consulta [README.md](../README.md)!
