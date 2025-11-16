use raylib::prelude::*;

/// Uniforms: datos que se pasan a los shaders y se mantienen constantes para todos los vértices
pub struct Uniforms {
    pub model_matrix: [[f32; 4]; 4],      // Matriz de modelo (transformación del objeto)
    pub view_matrix: [[f32; 4]; 4],       // Matriz de vista (cámara)
    pub projection_matrix: [[f32; 4]; 4], // Matriz de proyección
    pub viewport_matrix: [[f32; 4]; 4],   // Matriz de viewport (pantalla)
    pub time: f32,                         // Tiempo para animación
    pub noise_scale: f32,                  // Escala del ruido
    pub turbulence_intensity: f32,         // Intensidad de turbulencia
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_matrix: identity_matrix(),
            view_matrix: identity_matrix(),
            projection_matrix: identity_matrix(),
            viewport_matrix: identity_matrix(),
            time: 0.0,
            noise_scale: 1.0,
            turbulence_intensity: 1.0,
        }
    }
}

/// Crea una matriz identidad 4x4
fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de rotación en Y
pub fn create_rotation_y_matrix(angle: f32) -> [[f32; 4]; 4] {
    let cos = angle.cos();
    let sin = angle.sin();
    
    [
        [cos, 0.0, -sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de rotación en X
pub fn create_rotation_x_matrix(angle: f32) -> [[f32; 4]; 4] {
    let cos = angle.cos();
    let sin = angle.sin();
    
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, sin, 0.0],
        [0.0, -sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de escala
pub fn create_scale_matrix(scale: f32) -> [[f32; 4]; 4] {
    [
        [scale, 0.0, 0.0, 0.0],
        [0.0, scale, 0.0, 0.0],
        [0.0, 0.0, scale, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de traslación
pub fn create_translation_matrix(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de vista (cámara)
pub fn create_view_matrix(eye: Vector3, center: Vector3, up: Vector3) -> [[f32; 4]; 4] {
    let f = vector3_normalize(vector3_subtract(center, eye));
    let s = vector3_normalize(vector3_cross_product(f, up));
    let u = vector3_cross_product(s, f);

    [
        [s.x, s.y, s.z, -vector3_dot_product(s, eye)],
        [u.x, u.y, u.z, -vector3_dot_product(u, eye)],
        [-f.x, -f.y, -f.z, vector3_dot_product(f, eye)],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Crea una matriz de proyección perspectiva
pub fn create_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fov / 2.0).tan();
    
    [
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far)],
        [0.0, 0.0, -1.0, 0.0],
    ]
}

/// Crea una matriz de viewport
pub fn create_viewport_matrix(width: f32, height: f32) -> [[f32; 4]; 4] {
    [
        [width / 2.0, 0.0, 0.0, width / 2.0],
        [0.0, -height / 2.0, 0.0, height / 2.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Multiplica dos matrices 4x4
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

/// Multiplica una matriz 4x4 por un vector (usado para transformar vértices)
pub fn multiply_matrix_vector(matrix: &[[f32; 4]; 4], vector: Vector3) -> Vector3 {
    let x = matrix[0][0] * vector.x + matrix[0][1] * vector.y + matrix[0][2] * vector.z + matrix[0][3];
    let y = matrix[1][0] * vector.x + matrix[1][1] * vector.y + matrix[1][2] * vector.z + matrix[1][3];
    let z = matrix[2][0] * vector.x + matrix[2][1] * vector.y + matrix[2][2] * vector.z + matrix[2][3];
    let w = matrix[3][0] * vector.x + matrix[3][1] * vector.y + matrix[3][2] * vector.z + matrix[3][3];
    
    // Dividir por w para perspectiva correcta
    if w != 0.0 {
        Vector3::new(x / w, y / w, z / w)
    } else {
        Vector3::new(x, y, z)
    }
}

// Funciones auxiliares de vectores
fn vector3_subtract(a: Vector3, b: Vector3) -> Vector3 {
    Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z)
}

fn vector3_normalize(v: Vector3) -> Vector3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length > 0.0 {
        Vector3::new(v.x / length, v.y / length, v.z / length)
    } else {
        v
    }
}

fn vector3_cross_product(a: Vector3, b: Vector3) -> Vector3 {
    Vector3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

fn vector3_dot_product(a: Vector3, b: Vector3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
