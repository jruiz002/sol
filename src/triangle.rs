use crate::vertex::{Vertex, Fragment};
use raylib::prelude::*;

/// Calcula las coordenadas baricéntricas de un punto P respecto a un triángulo ABC
/// Las coordenadas baricéntricas son pesos que indican qué tan cerca está P de cada vértice
/// Retorna (w, v, u) donde w+v+u = 1.0 si P está dentro del triángulo
fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex) -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }
    
    let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}

/// RASTERIZACIÓN DE TRIÁNGULO
/// Convierte un triángulo en fragmentos (píxeles)
/// Usa coordenadas baricéntricas para interpolar atributos
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Obtener coordenadas transformadas (en pantalla)
    let a_x = v1.transformed_position.x;
    let b_x = v2.transformed_position.x;
    let c_x = v3.transformed_position.x;
    let a_y = v1.transformed_position.y;
    let b_y = v2.transformed_position.y;
    let c_y = v3.transformed_position.y;

    // Calcular bounding box del triángulo
    let min_x = a_x.min(b_x).min(c_x).floor() as i32;
    let min_y = a_y.min(b_y).min(c_y).floor() as i32;
    let max_x = a_x.max(b_x).max(c_x).ceil() as i32;
    let max_y = a_y.max(b_y).max(c_y).ceil() as i32;

    // Iterar sobre cada píxel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (w, v, u) = barycentric_coordinates(x as f32, y as f32, v1, v2, v3);

            // Si el punto está dentro del triángulo (todas las coordenadas son positivas)
            if w >= 0.0 && v >= 0.0 && u >= 0.0 {
                // Interpolar posición en espacio mundo usando coordenadas baricéntricas
                let world_pos = Vector3::new(
                    w * v1.world_position.x + v * v2.world_position.x + u * v3.world_position.x,
                    w * v1.world_position.y + v * v2.world_position.y + u * v3.world_position.y,
                    w * v1.world_position.z + v * v2.world_position.z + u * v3.world_position.z,
                );

                // Interpolar normal
                let normal = Vector3::new(
                    w * v1.normal.x + v * v2.normal.x + u * v3.normal.x,
                    w * v1.normal.y + v * v2.normal.y + u * v3.normal.y,
                    w * v1.normal.z + v * v2.normal.z + u * v3.normal.z,
                );

                // Crear fragmento con todos los datos interpolados
                let mut fragment = Fragment::new(
                    x as f32,
                    y as f32,
                    Vector3::new(1.0, 1.0, 1.0),
                    world_pos.z,
                );
                fragment.world_position = world_pos;
                fragment.normal = normal;
                
                fragments.push(fragment);
            }
        }
    }

    fragments
}
