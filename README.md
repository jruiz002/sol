# ⭐ Estrella Animada con Shaders

Proyecto de gráficas por computadora que simula una estrella/sol con animación procedural usando shaders y funciones de ruido.

![Demo de la Estrella](demo.gif)

## 📋 Descripción

Este proyecto implementa una estrella animada utilizando únicamente una esfera como geometría base. Toda la apariencia visual, efectos de superficie y animación se generan mediante shaders personalizados y funciones de ruido procedural.

### Características Principales

- ✅ **Framebuffer personalizado** - No usa el framebuffer de Raylib
- ✅ **Pipeline de renderizado completo** - Vertex shader, rasterización y fragment shader
- ✅ **Múltiples tipos de ruido** - Perlin, Turbulencia y Cellular noise
- ✅ **Animación continua** - Usando variable de tiempo (uniform)
- ✅ **Gradiente de temperatura** - Colores dinámicos basados en intensidad
- ✅ **Emisión variable** - Simula picos de energía y luminosidad
- ✅ **Distorsión de vértices** - Vertex shader modifica la geometría
- ✅ **Efecto de flare** - Brillo adicional en los bordes

## 🏗️ Arquitectura del Proyecto

### Estructura de Archivos

```
sol/
├── assets/
│   └── sphere.obj        # Modelo 3D de la esfera
├── src/
│   ├── main.rs           # Loop principal y configuración
│   ├── framebuffer.rs    # Framebuffer personalizado
│   ├── vertex.rs         # Estructuras de vértice y fragmento
│   ├── uniforms.rs       # Matrices de transformación
│   ├── noise.rs          # Funciones de ruido (Perlin, Cellular, etc.)
│   ├── shaders.rs        # Vertex y Fragment shaders
│   ├── triangle.rs       # Rasterización de triángulos
│   ├── obj_loader.rs     # Cargador de archivos OBJ
│   └── renderer.rs       # Pipeline de renderizado
├── Cargo.toml
└── README.md
```

## 🎨 Conceptos Implementados

### 1. Carga de Geometría desde Archivo OBJ

El proyecto carga la geometría de la esfera desde un archivo **OBJ** (`assets/sphere.obj`) en lugar de generarla proceduralmente. El cargador implementado en `obj_loader.rs`:

- **Lee el formato OBJ** línea por línea
- **Extrae vértices** (líneas que empiezan con `v`)
- **Extrae normales** (líneas que empiezan con `vn`)
- **Extrae caras/triángulos** (líneas que empiezan con `f`)
- **Calcula normales automáticamente** si el archivo no las incluye

```rust
// En main.rs
let obj_model = ObjModel::load("assets/sphere.obj")?;
let sphere_vertices = obj_model.vertices;
let sphere_indices = obj_model.indices;
```

**¿Por qué es importante?** Permite usar modelos 3D creados en herramientas profesionales (Blender, Maya, etc.) y es una práctica estándar en gráficas por computadora.

### 2. Framebuffer Personalizado

El **framebuffer** (`framebuffer.rs`) es un buffer de píxeles que almacena la imagen antes de mostrarla en pantalla. En lugar de usar el framebuffer nativo de Raylib, implementamos uno propio que:

- **Almacena píxeles manualmente** en un `Vec<Color>`
- **Permite control total** sobre cada píxel de la pantalla
- **Se actualiza** mediante `swap_buffers()` que convierte los píxeles a una textura
- **Se dibuja** con `draw_to_screen()` en el loop principal

**¿Por qué es importante?** Nos da control completo sobre el proceso de renderizado y es fundamental para implementar un pipeline gráfico desde cero.

```rust
pub struct Framebuffer {
    pub pixels: Vec<Color>,  // Array de píxeles
    pub width: u32,
    pub height: u32,
    // ...
}
```

### 2. Multiplicación de Matrices

Las **matrices de transformación** (`uniforms.rs`) son fundamentales para convertir coordenadas 3D en coordenadas 2D de pantalla. Implementamos:

#### Matrices Utilizadas:
- **Matriz de Modelo**: Rota y escala la estrella
- **Matriz de Vista**: Posiciona la cámara
- **Matriz de Proyección**: Convierte 3D a 2D (perspectiva)
- **Matriz de Viewport**: Mapea a coordenadas de pantalla

#### Multiplicación de Matrices:
```rust
pub fn multiply_matrices(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}
```

**¿Para qué se usa?** En el vertex shader, multiplicamos todas las matrices para transformar cada vértice:

```
Posición Final = Viewport × Proyección × Vista × Modelo × Vértice
```

Esto nos permite rotar la estrella, posicionar la cámara y proyectar todo a la pantalla.

### 3. Rasterización

La **rasterización** (`triangle.rs`) es el proceso de convertir triángulos en píxeles. Usamos **coordenadas baricéntricas** para:

1. **Determinar si un píxel está dentro del triángulo**
2. **Interpolar atributos** (posición, normal, color) entre los vértices
3. **Generar fragmentos** para cada píxel del triángulo

```rust
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    // Calcular bounding box
    // Para cada píxel en el box:
    //   - Calcular coordenadas baricéntricas
    //   - Si está dentro del triángulo, crear fragmento
    //   - Interpolar atributos usando las coordenadas
}
```

**Coordenadas Baricéntricas (w, v, u):**
- Si w ≥ 0, v ≥ 0, u ≥ 0 → el punto está dentro
- Se usan como pesos para interpolar: `valor = w*v1 + v*v2 + u*v3`

### 4. Pipeline de Renderizado

El **pipeline completo** (`renderer.rs`) sigue estas etapas:

```
Vértices 3D → Vertex Shader → Triángulos → Rasterización → Fragment Shader → Píxeles
```

#### Etapa 1: Vertex Shader
Transforma cada vértice y aplica distorsión:
```rust
pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // 1. Multiplicar por matrices (MVP)
    // 2. Aplicar distorsión usando ruido
    // 3. Convertir a coordenadas de pantalla
}
```

#### Etapa 2: Rasterización
Convierte triángulos en fragmentos (píxeles candidatos)

#### Etapa 3: Fragment Shader
Calcula el color final de cada píxel:
```rust
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // 1. Calcular ruido (Perlin, turbulencia)
    // 2. Aplicar manchas solares (cellular noise)
    // 3. Calcular pulsaciones
    // 4. Convertir intensidad a color (temperatura)
    // 5. Agregar emisión y flare
}
```

## 🌊 Funciones de Ruido

### Perlin Noise
Genera patrones suaves y naturales. Se usa como base para la superficie de la estrella.

```rust
pub fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    // Usa interpolación suave entre gradientes aleatorios
    // Produce valores entre 0.0 y 1.0
}
```

**Aplicación:** Crea variaciones suaves en la superficie de la estrella.

### Turbulencia
Combina múltiples octavas de Perlin noise para crear patrones complejos.

```rust
pub fn turbulence(x: f32, y: f32, z: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += amplitude * perlin_noise(x * frequency, y * frequency, z * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    value
}
```

**Aplicación:** Simula la actividad turbulenta de la superficie solar.

### Cellular/Worley Noise
Crea patrones celulares basados en distancias a puntos aleatorios.

```rust
pub fn cellular_noise(pos: Vector3, scale: f32) -> f32 {
    // Encuentra la distancia al punto de característica más cercano
    // Produce patrones tipo "celdas" o "burbujas"
}
```

**Aplicación:** Genera las "manchas solares" oscuras en la estrella.

## 🎨 Gradiente de Temperatura

El color de la estrella cambia según su intensidad, simulando el espectro de cuerpo negro:

| Intensidad | Color | Temperatura Simulada |
|-----------|-------|---------------------|
| 0.0 - 0.3 | Negro → Rojo oscuro | Manchas solares frías |
| 0.3 - 0.5 | Rojo oscuro → Rojo | Zonas templadas |
| 0.5 - 0.7 | Rojo → Naranja | Temperatura media |
| 0.7 - 0.85 | Naranja → Amarillo | Zonas calientes |
| 0.85 - 1.0 | Amarillo → Blanco | Zonas muy calientes |

```rust
fn temperature_to_color(intensity: f32) -> Vector3 {
    // Mapea intensidad a color RGB
    // Simula espectro de temperatura de estrella
}
```

## ⚡ Efectos Implementados

### 1. Emisión Variable
Simula picos de energía usando funciones seno:
```rust
let emission_boost = ((pos.x + pos.y + pos.z) * 10.0 + time * 5.0).sin() * 0.2 + 1.0;
```

### 2. Distorsión de Vértices (Vertex Shader)
Desplaza los vértices a lo largo de sus normales:
```rust
let noise_offset = turbulence(...) * 0.1 * uniforms.turbulence_intensity;
let distorted_position = position + normal * noise_offset;
```
**Efecto:** La superficie parece "vibrar" y tener actividad.

### 3. Efecto de Flare
Agrega brillo adicional en los bordes:
```rust
let distance_from_center = (pos.x² + pos.y² + pos.z²).sqrt();
let flare = (1.0 - distance_from_center).max(0.0) * 0.3;
```

### 4. Pulsación
Simula el latido de la estrella:
```rust
let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.2 + 0.8;
```

## 🚀 Compilación y Ejecución

### Requisitos
- Rust (1.70 o superior)
- Raylib 5.0

### Instalar Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compilar y Ejecutar
```bash
# Clonar el repositorio
git clone <tu-repo>
cd sol

# Compilar en modo release (más rápido)
cargo build --release

# Ejecutar
cargo run --release
```

## 🎮 Controles

- **ESC**: Salir de la aplicación

## 📊 Parámetros Ajustables

En `main.rs` puedes modificar:

```rust
// Resolución de la esfera
let slices = 40;  // Mayor = más detalle
let stacks = 40;

// Parámetros de ruido
uniforms.noise_scale = 2.0;  // Escala del ruido
uniforms.turbulence_intensity = 0.8;  // Intensidad de turbulencia
```

## 🏆 Criterios Cumplidos

| Criterio | Puntos | Implementación |
|----------|--------|----------------|
| Creatividad visual y realismo | 30 | Gradiente de temperatura, efectos de flare |
| Complejidad del shader | 40 | Perlin + Turbulencia + Cellular noise |
| Animación continua | 20 | Variable `time` en fragment shader |
| Uso de Perlin/Cellular noise | 20 | `noise.rs` - múltiples funciones |
| Emisión variable | 15 | Picos de energía con seno |
| Distorsión en Vertex Shader | 15 | Desplazamiento por normal |
| Control de color por temperatura | 20 | Función `temperature_to_color` |
| Documentación | 10 | Este README |

**Total:** 170 puntos

## 📚 Referencias Técnicas

- **Perlin Noise**: Algoritmo de Ken Perlin para ruido coherente
- **Coordenadas Baricéntricas**: Para interpolación en triángulos
- **Pipeline Gráfico**: Vertex → Rasterización → Fragment
- **Matrices de Transformación**: Modelo-Vista-Proyección (MVP)

## 👨‍💻 Autor

Proyecto desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala

## 📄 Licencia

MIT License - Libre para uso educativo
