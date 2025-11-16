use raylib::prelude::*;

/// Estructura que representa un vértice en el espacio 3D
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,              // Posición original en espacio local
    pub normal: Vector3,                // Normal del vértice
    pub transformed_position: Vector3,   // Posición después de transformaciones
    pub world_position: Vector3,         // Posición en espacio mundo
}

impl Vertex {
    pub fn new(position: Vector3, normal: Vector3) -> Self {
        Self {
            position,
            normal,
            transformed_position: position,
            world_position: position,
        }
    }
}

/// Estructura que representa un fragmento (píxel) durante la rasterización
#[derive(Clone, Debug)]
pub struct Fragment {
    pub position: Vector2,       // Posición en pantalla
    pub color: Vector3,          // Color base del fragmento
    pub world_position: Vector3, // Posición en espacio mundo
    pub normal: Vector3,         // Normal interpolada
    pub depth: f32,              // Profundidad del fragmento
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Vector3, depth: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            color,
            world_position: Vector3::new(x, y, depth),
            normal: Vector3::new(0.0, 0.0, 1.0),
            depth,
        }
    }
}
