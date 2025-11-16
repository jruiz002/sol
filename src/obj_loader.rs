use raylib::prelude::*;
use crate::vertex::Vertex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Estructura para almacenar la geometría cargada desde un archivo OBJ
pub struct ObjModel {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
}

impl ObjModel {
    /// Carga un archivo OBJ y retorna la geometría
    pub fn load(path: &str) -> Result<Self, String> {
        let file = File::open(path)
            .map_err(|e| format!("Error abriendo archivo {}: {}", path, e))?;
        
        let reader = BufReader::new(file);
        
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        
        // Leer el archivo línea por línea
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error leyendo línea: {}", e))?;
            let line = line.trim();
            
            // Ignorar líneas vacías y comentarios
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                // Vértice: v x y z
                "v" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>()
                            .map_err(|e| format!("Error parseando x: {}", e))?;
                        let y = parts[2].parse::<f32>()
                            .map_err(|e| format!("Error parseando y: {}", e))?;
                        let z = parts[3].parse::<f32>()
                            .map_err(|e| format!("Error parseando z: {}", e))?;
                        positions.push(Vector3::new(x, y, z));
                    }
                }
                // Normal: vn x y z
                "vn" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>()
                            .map_err(|e| format!("Error parseando normal x: {}", e))?;
                        let y = parts[2].parse::<f32>()
                            .map_err(|e| format!("Error parseando normal y: {}", e))?;
                        let z = parts[3].parse::<f32>()
                            .map_err(|e| format!("Error parseando normal z: {}", e))?;
                        normals.push(Vector3::new(x, y, z));
                    }
                }
                // Cara: f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
                "f" => {
                    if parts.len() >= 4 {
                        for i in 1..=3 {
                            let face_part = parts[i];
                            let values: Vec<&str> = face_part.split('/').collect();
                            
                            if !values.is_empty() {
                                // Los índices en OBJ empiezan en 1, no en 0
                                let vertex_index = values[0].parse::<usize>()
                                    .map_err(|e| format!("Error parseando índice: {}", e))? - 1;
                                indices.push(vertex_index);
                            }
                        }
                    }
                }
                _ => {} // Ignorar otras líneas (vt, etc.)
            }
        }
        
        // Si no hay normales en el archivo, calcularlas
        if normals.is_empty() {
            normals = Self::calculate_normals(&positions);
        }
        
        // Asegurar que tenemos suficientes normales
        while normals.len() < positions.len() {
            normals.push(Vector3::new(0.0, 1.0, 0.0));
        }
        
        // Crear vértices combinando posiciones y normales
        let mut vertices = Vec::new();
        for (i, pos) in positions.iter().enumerate() {
            let normal = if i < normals.len() {
                normals[i]
            } else {
                // Si no hay normal, usar la posición normalizada (para esfera)
                let length = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                if length > 0.0 {
                    Vector3::new(pos.x / length, pos.y / length, pos.z / length)
                } else {
                    Vector3::new(0.0, 1.0, 0.0)
                }
            };
            vertices.push(Vertex::new(*pos, normal));
        }
        
        println!("✅ OBJ cargado: {} vértices, {} índices", vertices.len(), indices.len());
        
        Ok(ObjModel { vertices, indices })
    }
    
    /// Calcula normales si el archivo OBJ no las tiene
    fn calculate_normals(positions: &[Vector3]) -> Vec<Vector3> {
        positions.iter().map(|pos| {
            let length = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
            if length > 0.0 {
                Vector3::new(pos.x / length, pos.y / length, pos.z / length)
            } else {
                Vector3::new(0.0, 1.0, 0.0)
            }
        }).collect()
    }
}
