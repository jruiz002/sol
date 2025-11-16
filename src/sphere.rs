use raylib::prelude::*;
use crate::vertex::Vertex;
use std::f32::consts::PI;

/// Genera una esfera usando coordenadas esféricas
/// Retorna un vector de vértices que forman la geometría de la esfera
pub fn create_sphere(radius: f32, slices: u32, stacks: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    // Generar vértices usando coordenadas esféricas
    for stack in 0..=stacks {
        let phi = PI * stack as f32 / stacks as f32;
        
        for slice in 0..=slices {
            let theta = 2.0 * PI * slice as f32 / slices as f32;
            
            // Conversión de coordenadas esféricas a cartesianas
            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.cos();
            let z = radius * phi.sin() * theta.sin();
            
            let position = Vector3::new(x, y, z);
            
            // La normal de una esfera es el vector normalizado desde el centro
            let normal = Vector3::new(x / radius, y / radius, z / radius);
            
            vertices.push(Vertex::new(position, normal));
        }
    }
    
    vertices
}

/// Genera los índices para dibujar la esfera como triángulos
/// Cada cuadrilátero de la grilla se divide en 2 triángulos
pub fn create_sphere_indices(slices: u32, stacks: u32) -> Vec<usize> {
    let mut indices = Vec::new();
    
    for stack in 0..stacks {
        for slice in 0..slices {
            let current = (stack * (slices + 1) + slice) as usize;
            let next = current + slices as usize + 1;
            
            // Primer triángulo del quad
            indices.push(current);
            indices.push(next);
            indices.push(current + 1);
            
            // Segundo triángulo del quad
            indices.push(current + 1);
            indices.push(next);
            indices.push(next + 1);
        }
    }
    
    indices
}
