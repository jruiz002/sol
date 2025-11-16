# ⭐ Estrella Animada con Shaders

**Proyecto de Gráficas por Computadora**  
Universidad del Valle de Guatemala - 2025

---

## 🎬 Demostración en Video

[![Ver demostración en YouTube](https://img.youtube.com/vi/ZepfgJbB5-g/maxresdefault.jpg)](https://youtu.be/ZepfgJbB5-g)

**[▶️ Ver video completo en YouTube](https://youtu.be/ZepfgJbB5-g)**

---

![Demo de la Estrella](demo.gif)

## 📋 Descripción del Proyecto

Este proyecto simula una **estrella/sol animado** con efectos realistas de superficie solar. A diferencia de usar texturas prediseñadas, toda la apariencia visual se genera **proceduralmente** mediante shaders y funciones matemáticas de ruido.

### 🎯 Objetivo

Implementar un pipeline de renderizado 3D completo desde cero, sin usar funciones de alto nivel de Raylib, para:
- Entender el funcionamiento interno de los motores gráficos
- Aplicar conceptos de álgebra lineal (matrices, vectores)
- Practicar programación de shaders y técnicas procedurales

### ✨ Características Visuales

| Efecto | Descripción |
|--------|-------------|
| 🎨 **Gradiente de Temperatura** | Colores que van de negro → rojo → naranja → amarillo → blanco |
| 🌊 **Turbulencia Solar** | Superficie dinámica con patrones de ruido animados |
| ⚫ **Manchas Solares** | Regiones más oscuras generadas con Cellular Noise |
| 💫 **Emisión Variable** | Picos de brillo que simulan actividad energética |
| 🌟 **Efecto Flare** | Resplandor en los bordes de la estrella |
| 🔄 **Animación Continua** | Todo se mueve y cambia con el tiempo |

## � Conceptos Educativos Aplicados

Este proyecto demuestra comprensión profunda de los siguientes conceptos de gráficas por computadora:

### 1️⃣ **Pipeline de Renderizado 3D**

```
📦 Geometría (OBJ)
    ↓
🔧 Vertex Shader (Transformaciones + Distorsión)
    ↓
🔺 Ensamblado de Primitivas (Triángulos)
    ↓
📐 Rasterización (Coordenadas Baricéntricas)
    ↓
🎨 Fragment Shader (Colores + Efectos)
    ↓
📺 Framebuffer (Píxeles finales)
```

**Aprendizaje:** Cómo funciona internamente un motor gráfico, paso por paso.

---

### 2️⃣ **Framebuffer Personalizado** (`framebuffer.rs`)

En lugar de dejar que Raylib maneje los píxeles, implementamos nuestro propio buffer:

```rust
pub struct Framebuffer {
    pub pixels: Vec<Color>,  // Array manual de píxeles
    pub width: u32,
    pub height: u32,
}
```

**¿Por qué es importante?**
- ✅ Control total sobre cada píxel de la pantalla
- ✅ Entender cómo se almacenan las imágenes en memoria
- ✅ Base para implementar efectos post-procesamiento

**Dónde se usa:**
- `clear()` - Limpia todos los píxeles
- `point(x, y, color)` - Escribe un píxel calculado por el fragment shader
- `swap_buffers()` - Convierte el array de píxeles a textura de Raylib

---

### 3️⃣ **Multiplicación de Matrices** (`uniforms.rs`)

Las matrices transforman objetos 3D en coordenadas 2D de pantalla. Implementamos la multiplicación manualmente:

```rust
pub fn multiply_matrices(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // Multiplicación manual matriz 4x4
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}
```

**Transformación MVP (Model-View-Projection):**

```
Vértice Local → [Modelo] → Mundo → [Vista] → Cámara → [Proyección] → Clip → [Viewport] → Pantalla
```

**¿Para qué sirve cada matriz?**
- 🔄 **Model**: Rota, escala y mueve la estrella
- 📷 **View**: Posiciona la cámara en el espacio
- 📐 **Projection**: Aplica perspectiva (objetos lejanos se ven pequeños)
- 🖥️ **Viewport**: Mapea a coordenadas de pantalla (0 a 800, 0 a 600)

**Aprendizaje:** Álgebra lineal aplicada - transformaciones geométricas en 3D.

---

### 4️⃣ **Rasterización con Coordenadas Baricéntricas** (`triangle.rs`)

Convertimos triángulos 3D en píxeles 2D:

```rust
fn barycentric_coordinates(p: Point, a: Vertex, b: Vertex, c: Vertex) -> (w, v, u) {
    // Calcula pesos para interpolar atributos
}
```

**¿Cómo funciona?**
1. Para cada píxel de la pantalla, calculamos sus coordenadas baricéntricas (w, v, u)
2. Si `w ≥ 0 && v ≥ 0 && u ≥ 0` → el píxel está dentro del triángulo
3. Usamos w, v, u como pesos para interpolar color, normal, posición:

```
color_final = w × color_A + v × color_B + u × color_C
```

**Aprendizaje:** Geometría computacional - interpolación de atributos.

---

### 5️⃣ **Funciones de Ruido Procedural** (`noise.rs`)

Generamos patrones naturales sin usar imágenes:

#### **Perlin Noise**
```rust
pub fn perlin_noise(x: f32, y: f32, z: f32) -> f32
```
- Crea patrones suaves y continuos
- Usa interpolación entre gradientes aleatorios
- **Uso:** Base de la textura de superficie

#### **Turbulencia (Fractal Brownian Motion)**
```rust
pub fn turbulence(x, y, z, octaves: 4) -> f32 {
    // Suma de 4 capas de Perlin Noise con diferentes frecuencias
}
```
- Combina múltiples octavas para complejidad
- **Uso:** Simula actividad turbulenta de la superficie solar

#### **Cellular/Worley Noise**
```rust
pub fn cellular_noise(pos: Vector3, scale: f32) -> f32
```
- Calcula distancia a puntos aleatorios
- Crea patrones celulares
- **Uso:** Genera manchas solares oscuras

**Aprendizaje:** Generación procedural - crear texturas con matemáticas.

---

### 6️⃣ **Shaders Personalizados** (`shaders.rs`)

#### **Vertex Shader**
Transforma y distorsiona cada vértice:

```rust
pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // 1. Aplicar transformación MVP
    let mvp = projection × view × model;
    let screen_pos = mvp × vertex.position;
    
    // 2. Distorsión procedural (superficie burbujeante)
    let noise = turbulence(position + time);
    distorted_pos = position + normal × noise;
}
```

**Efecto:** La superficie de la estrella se mueve y ondula.

#### **Fragment Shader**
Calcula el color de cada píxel:

```rust
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // 1. Ruido base
    let turb = turbulence(pos × 2.0 + time × 0.3);
    
    // 2. Manchas solares
    let spots = cellular_noise(pos);
    
    // 3. Intensidad combinada
    let intensity = turb - spots;
    
    // 4. Mapear a color (temperatura)
    let color = temperature_to_color(intensity);
    
    // 5. Emisión variable (picos de energía)
    let emission = sin(pos.sum() + time×5) × 0.2 + 1.0;
    
    // 6. Flare en bordes
    let flare = (1.0 - distance_to_center) × 0.3;
    
    return color × emission + flare;
}
```

**Aprendizaje:** Programación de shaders - lógica de renderizado pixel por pixel.

---

### 7️⃣ **Carga de Modelos 3D** (`obj_loader.rs`)

Parser de archivos OBJ (formato estándar de la industria):

```rust
// Lee vértices: v x y z
// Lee normales: vn nx ny nz  
// Lee caras: f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
```

**Aprendizaje:** Parsing de archivos - leer formatos de datos 3D.

---

## 🏗️ Estructura del Proyecto

```
sol/
├── assets/
│   └── sphere.obj        # Geometría (vértices y triángulos)
├── src/
│   ├── main.rs           # ⚙️ Loop principal
│   ├── framebuffer.rs    # 📺 Buffer de píxeles
│   ├── vertex.rs         # 📍 Estructuras de datos
│   ├── uniforms.rs       # 🔢 Matrices y transformaciones
│   ├── noise.rs          # 🌊 Perlin, Cellular, Turbulencia
│   ├── shaders.rs        # 🎨 Vertex y Fragment shaders
│   ├── triangle.rs       # 📐 Rasterización
│   ├── obj_loader.rs     # 📦 Cargador de OBJ
│   └── renderer.rs       # 🔄 Pipeline completo
└── Cargo.toml
```

## 🔬 Detalles Técnicos de Implementación

### 📊 Parámetros del Shader (Uniforms)

Los **uniforms** son variables globales que se pasan a todos los vértices y fragmentos:

| Uniform | Tipo | Valor | Descripción |
|---------|------|-------|-------------|
| `model_matrix` | mat4x4 | Rotación Y × Rotación X × Escala | Transforma la estrella |
| `view_matrix` | mat4x4 | lookAt(eye, center, up) | Posición de cámara |
| `projection_matrix` | mat4x4 | Perspectiva (FOV 45°) | Proyección 3D→2D |
| `viewport_matrix` | mat4x4 | Escala a 800×600 | Mapeo a pantalla |
| `time` | float | `get_time()` | Tiempo para animación |
| `noise_scale` | float | 2.0 | Frecuencia del ruido |
| `turbulence_intensity` | float | 0.8 | Fuerza de distorsión |

### �️ Gradiente de Temperatura (Espectro de Cuerpo Negro)

```
Intensidad:  0.0      0.3      0.5      0.7      0.85     1.0
             │        │        │        │        │        │
Color:       ⚫ ───→  🔴 ───→  🟠 ───→  🟡 ───→  ⚪
             Negro    Rojo     Naranja  Amarillo Blanco
Temp:        Frío               ~5000K             Muy caliente
```

### ⚡ Efectos Visuales en Acción

1. **🔄 Distorsión de Vértices**
   ```rust
   noise_offset = turbulence(pos + time) × 0.1
   new_position = position + normal × noise_offset
   ```
   → Superficie "burbujeante"

2. **💫 Emisión Variable**
   ```rust
   emission = sin(pos.sum() + time×5) × 0.2 + 1.0
   ```
   → Picos de energía (80% a 120% de brillo)

3. **🌟 Flare en Bordes**
   ```rust
   flare = (1.0 - distance_to_center) × 0.3
   ```
   → Resplandor exterior

4. **💓 Pulsación Global**
   ```rust
   pulse = sin(time×2) × 0.1 + 0.9
   ```
   → "Latido" de la estrella

## 🚀 Cómo Ejecutar el Proyecto

### Requisitos Previos
- **Rust** 1.70+ ([Instalar aquí](https://rustup.rs/))
- **Raylib** 5.0 (se instala automáticamente con Cargo)

### Paso a Paso

```bash
# 1. Clonar el repositorio
git clone https://github.com/jruiz002/sol.git
cd sol

# 2. Compilar (modo optimizado - recomendado)
cargo build --release

# 3. Ejecutar
cargo run --release
```

**💡 Nota:** El modo `--release` es 10x más rápido que el modo debug.

### 🎮 Controles

| Tecla | Acción |
|-------|--------|
| `ESC` | Salir |

---

## 🛠️ Personalización y Experimentación

### Modificar Parámetros en `main.rs`

```rust
// Intensidad de los efectos
uniforms.noise_scale = 2.0;           // ↑ más detalle, ↓ más suave
uniforms.turbulence_intensity = 0.8;  // ↑ más agitado, ↓ más calmado

// Velocidad de rotación
rotation_angle += 0.3 * delta_time;   // Cambiar el 0.3

// Tamaño de la estrella
let scale = create_scale_matrix(1.5); // Cambiar el 1.5
```

### Cambiar Colores en `shaders.rs`

```rust
// Estrella azul (muy caliente)
fn temperature_to_color(intensity: f32) -> Vector3 {
    Vector3::new(
        intensity * 0.5,  // Rojo
        intensity * 0.7,  // Verde  
        intensity * 1.0   // Azul (dominante)
    )
}
```

## 📚 Conceptos de Gráficas por Computadora Demostrados

### Transformaciones Geométricas
- ✅ Multiplicación de matrices 4×4
- ✅ Composición de transformaciones (MVP)
- ✅ Sistemas de coordenadas (local, mundo, cámara, clip, pantalla)

### Rasterización
- ✅ Coordenadas baricéntricas
- ✅ Interpolación de atributos
- ✅ Bounding box optimization

### Shaders Programables
- ✅ Vertex shader (transformación + efectos)
- ✅ Fragment shader (iluminación + texturización procedural)
- ✅ Uniforms (variables globales)

### Técnicas Procedurales
- ✅ Perlin Noise (ruido coherente)
- ✅ Fractal Brownian Motion (turbulencia)
- ✅ Cellular/Worley Noise (patrones celulares)

### Pipeline Gráfico
- ✅ Vertex processing
- ✅ Primitive assembly
- ✅ Rasterization
- ✅ Fragment processing
- ✅ Framebuffer operations

---

## 🎓 Aprendizajes Clave

1. **No todo es "plug and play"** - Implementar el pipeline desde cero da verdadera comprensión
2. **Las matemáticas importan** - Álgebra lineal es la base de los gráficos 3D
3. **El ruido procedural es poderoso** - Patrones infinitos sin imágenes
4. **Los shaders son mini-programas** - Se ejecutan millones de veces por frame
5. **La optimización importa** - Mode release vs debug: 10x diferencia

## 👨‍💻 Autor

**José Ruiz**  
Universidad del Valle de Guatemala  
Gráficas por Computadora - 2025

